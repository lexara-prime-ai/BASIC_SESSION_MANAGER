use tokio::main;

use models::session::Session;

mod models;
mod utils;

#[main]
async fn main() {
    logger!(Session::new("session").unwrap());
}
