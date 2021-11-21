// this file is an example instance of the actual server, see server.rs for the implementation

mod server;
mod utils;

use server::Namikare;
use tokio;

#[tokio::main]
async fn main() {
    let server = Namikare::new("127.0.0.1:9292".to_owned());
    server.start().await;
}
