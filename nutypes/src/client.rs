use bytes::Bytes;
use http::{Method, StatusCode};
use thiserror::Error;

use crate::models::{GetProductError, Product, ProductId};

pub trait ApiError {
    fn status_code(&self) -> StatusCode;
}

#[derive(Debug, Error)]
pub enum ApiClientError<E> {
    /// This can only occur when a invalid base URL was provided.
    #[error("An invalid URL was provided: {0}")]
    ParseError(#[from] url::ParseError),

    /// An error occurred in reqwest.
    #[error("An error occurred while making the request: {0}")]
    ClientError(#[from] reqwest::Error),

    /// An error returned from the service. These errors are specific to the
    /// endpoint that was called.
    #[error(transparent)]
    ServiceError(E),

    /// A response was received, but we were unable to deserialize it. The
    /// status code and the receive body are returned.
    #[error("API returned an unknown response: Status: {0}, Body: {1:?}")]
    InvalidResponse(StatusCode, Bytes),
}

impl<E> ApiClientError<E> {
    pub fn invalid_response(status_code: StatusCode, body: Bytes) -> Self {
        Self::InvalidResponse(status_code, body)
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    client: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn client(mut self, client: reqwest::Client) -> ClientBuilder {
        self.client = Some(client);
        self
    }

    pub fn default_client() -> reqwest::Client {
        reqwest::Client::new()
    }

    pub fn build(self, base_url: url::Url) -> Client {
        let client = self.client.unwrap_or_else(Self::default_client);
        Client { base_url, client }
    }
}

pub struct Client {
    base_url: url::Url,

    client: reqwest::Client,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Internals of sending a request. Should only be used by internal
    /// functions.
    async fn send<T, E>(&self, method: Method, path: &str) -> Result<T, ApiClientError<E>>
    where
        T: serde::de::DeserializeOwned,
    {
        let u = self.base_url.join(path)?;

        let request = self.client.request(method, u);

        // Extract Trace headers from our current context and add them to the
        // outgoing request.

        let response = request.send().await?;

        // Copy the status code here in case we are unable to parse the response as
        // the Ok or Err variant.
        let status_code = response.status();

        // Read the entire response into a local buffer.
        let body = response.bytes().await?;

        // Try to parse the result as T.
        match serde_json::from_slice::<T>(&body) {
            Ok(result) => Ok(result),
            Err(_err) => Err(ApiClientError::invalid_response(status_code, body)),
        }
    }

    pub async fn get_product(
        &self,
        product_id: &ProductId,
    ) -> Result<Product, ApiClientError<GetProductError>> {
        let path = format!("products/{}-asb", product_id);
        self.send(Method::GET, &path).await
    }
}
