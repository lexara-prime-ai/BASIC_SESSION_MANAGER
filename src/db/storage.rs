use bson::oid::ObjectId;
use bson::Document;
use chrono::Utc;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

use crate::logger;
use crate::models::session::Session;
use anyhow::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub token: Option<String>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<Utc>,
}

impl SessionData {
    pub async fn store() -> Result<InsertOneResult, Error> {
        let conn_str = std::env::var("MONGODB_URI").expect("[MONGODB_URI] must be set.");
        let options = ClientOptions::parse(conn_str).await?;
        let client = Client::with_options(options)?;
        let database_name = std::env::var("DB_NAME").expect("[DB_NAME] to be set.");
        let collection_name =
            std::env::var("COLLECTION_NAME").expect("[COLLECTION_NAME] must be set.");

        // Establish connection & Retrieve existing sessions.
        let sessions = client
            .database(&database_name)
            .collection::<Document>(&collection_name);

        // Instantiate session data.
        let session = Session::new("session").unwrap();

        let payload = bson::to_bson(&session)?;
        let doc = payload.as_document().unwrap();
        let result = sessions.insert_one(doc.to_owned()).await?;

        logger!(result);

        Ok(result)
    }
}
