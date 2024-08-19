#![allow(unused)]
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::logger;
use anyhow::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub session_id: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub token: Option<String>,
}

impl Session {
    pub fn new(sub: &str) -> Result<Self, Error> {
        // Randomness.
        let mut rng = thread_rng();
        let mut nums: Vec<i32> = (1..10).collect();
        let mut alphabet: Vec<char> = ('a'..='z').collect();

        // Shuffle contents.
        nums.shuffle(&mut rng);
        alphabet.shuffle(&mut rng);

        let character_string = alphabet.into_iter().collect::<String>();
        let numbers = nums.iter().map(|&val| val.to_string()).collect::<String>();

        let session_id = format!("{}{}", character_string, numbers);

        let claims = Claims {
            sub: sub.to_owned(),
            session_id,
            exp: 3 * 60 * 60, // Valid for 3hrs.
        };

        // Generate token.
        let secret = std::env::var("SECRET_KEY").expect("[SECRET_KEY] must be set.");
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let token = encode(&Header::default(), &claims, &encoding_key)?;

        Ok(Self { token: Some(token) })
    }
}
