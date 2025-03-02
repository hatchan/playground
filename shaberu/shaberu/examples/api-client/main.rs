use shaberu::api::client;
use shaberu::models::{GetRoomRequest, RoomId};

#[tokio::main]
pub async fn main() {
    let api_client = client::builder().build();

    let room_id = RoomId::new("66e71b6a39ac93c82aeae23c615d4532").unwrap();
    let req = GetRoomRequest { room_id };

    match api_client.get_room(req).await {
        Ok(resp) => println!("Room: {}", resp.room.room_id),
        Err(err) => eprintln!("unable to retrieve room: {:?}", err),
    }
}
