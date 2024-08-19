#![allow(unused)]
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use rand::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub token: Option<String>,
}

impl Session {
    pub fn new() -> Self {
        // Randomness.
        let mut rng = thread_rng();
        let mut nums: Vec<i32> = (1..50).collect();
        let mut alphabet: Vec<char> = ('a'..='z').collect();

        // Shuffle contents.
        let random_numbers = nums.shuffle(&mut rng);
        let random_characters = alphabet.shuffle(&mut rng);

        let character_string = alphabet.into_iter().collect::<String>();
        let numbers = nu

        println!("{:?}", ses);

        // let session_id = 


        // let claims = Claims {
        //     sub: 
        // };

        // let token = encode(&Header::default(), &)
        Self {
            token: Some("".to_owned())
        }
    }
}