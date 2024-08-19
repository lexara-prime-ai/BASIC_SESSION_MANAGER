/*
    Project Title: A basic Session Manager (with MongoDB)
    Author: Irfan M. Ghat
    Year: 2024
*/

use db::storage::SessionData;
use tokio::main;

mod db;
mod models;
mod utils;

#[main]
async fn main() {
    SessionData::store().await.unwrap();
}
