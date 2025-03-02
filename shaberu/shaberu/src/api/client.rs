use crate::models::{GetRoomError, GetRoomRequest, GetRoomResponse};

pub struct ApiClient {}

impl ApiClient {
    pub async fn get_room(
        &self,
        req: GetRoomRequest,
    ) -> Result<GetRoomResponse, ApiClientError<GetRoomError>> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ApiClientError<E> {
    ServiceError(E),
}

pub fn builder() -> ApiClientBuilder {
    ApiClientBuilder::new()
}

pub struct ApiClientBuilder {}

impl ApiClientBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> ApiClient {
        ApiClient {}
    }
}
