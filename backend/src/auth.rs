use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use argon2::{password_hash::{ rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString }, Argon2};

use crate::utils;

pub trait JwtClaims {
    /// Get subject id
    fn get_sub(&self) -> String;
    /// Get expiration timestamp
    fn get_exp(&self) -> u64;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: u64,    // Expiration time (Unix timestamp)
}

impl JwtClaims for Claims {
    fn get_sub(&self) -> String  {
        self.sub.to_owned()
    }

    fn get_exp(&self) -> u64  {
        self.exp.clone()
    }
}


pub fn hash_password(argon2_config: &Argon2, password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    argon2_config.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_password(argon2_config: &Argon2, password: &str, hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed_password).unwrap();
    match argon2_config.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => true,
        _ => false,
    }
}


pub fn encode_jwt<C: Serialize + JwtClaims>(
    secret_key: &str, 
    claims: &C
) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(secret_key.as_ref());
    encode(&header, claims, &encoding_key)
}

pub fn decode_jwt<C: DeserializeOwned + JwtClaims>(
    secret_key: &str, 
    token: &str
) -> Result<C, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
    decode::<C>(token, &decoding_key, &validation).map(|data| data.claims)
}
pub fn authorize_jwt<C: DeserializeOwned + JwtClaims>(
    secret_key: &str, 
    token: &str
) -> Result<String, ()> {
    // TODO add logic for blacklisting tokens
    return match decode_jwt::<C>(secret_key, token) {
        Ok(claims) => {
            if claims.get_exp() < utils::get_unix_timestamp() {
                return Err(());
            } 

            Ok(claims.get_sub())
        },
        Err(_) => Err(())
    }
}