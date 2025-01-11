use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305, KeyInit,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const KEY: &[u8] = b"01234567890123456789012345678901";
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    name: String,
    #[serde(rename = "private_age")]
    age: u8,
    date_of_birth: DateTime<Utc>,
    skills: Vec<String>,
    state: WokeState,
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    data: Vec<u8>,
    // #[serde(
    //     serialize_with = "serialize_encrypt",
    //     deserialize_with = "deserialize_decrypt"
    // )]
    // sensitive: String,
    #[serde_with(as = "DisplayFromStr")]
    sensitive: SensitiveData,
    #[serde_as(as = "DisplayFromStr")]
    url: http::Uri,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SensitiveData(String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
enum WokeState {
    Working(String),
    OnLeave(DateTime<Utc>),
    Terminated(u32),
}

fn main() -> Result<()> {
    let _state = WokeState::Working("Rust Engineer".to_string());
    let state1 = WokeState::OnLeave(Utc::now());
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        date_of_birth: "1990-01-01T00:00:00Z".parse()?,
        skills: vec!["Rust".to_string(), "Python".to_string()],
        state: state1,
        data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        sensitive: SensitiveData::new("top secret"),
        url: "https://example.com".parse()?,
    };

    let json = serde_json::to_string(&user)?;
    println!("{json}");

    let user1: User = serde_json::from_str(&json)?;
    println!("{user1:?}");

    Ok(())
}

fn b64_encode<S>(data: &Vec<u8>, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = URL_SAFE_NO_PAD.encode(data);
    serializer.serialize_str(&encoded)
}

fn b64_decode<'de, D>(deserializer: D) -> std::result::Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    URL_SAFE_NO_PAD
        .decode(&encoded)
        .map_err(serde::de::Error::custom)
}

#[allow(dead_code)]
fn serialize_encrypt<S>(data: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encrypted = encrypt(data.as_bytes()).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&encrypted)
}

#[allow(dead_code)]
fn deserialize_decrypt<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encrypted = String::deserialize(deserializer)?;
    let decrypted = decrypt(&encrypted).map_err(serde::de::Error::custom)?;
    let decrypted = String::from_utf8(decrypted).map_err(serde::de::Error::custom)?;
    Ok(decrypted)
}

/// encrypt with chacha20poly1305 and then encode with base64
fn encrypt(data: &[u8]) -> Result<String> {
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data).unwrap();
    let nonce_cypertext: Vec<_> = nonce.iter().copied().chain(ciphertext).collect();
    let encoded = URL_SAFE_NO_PAD.encode(nonce_cypertext);
    Ok(encoded)
}

/// decode with base64 and then decrypt with chacha20poly1305
fn decrypt(encoded: &str) -> Result<Vec<u8>> {
    let decoded = URL_SAFE_NO_PAD.decode(encoded.as_bytes())?;
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = decoded[..12].into();
    let decrypted = cipher.decrypt(nonce, &decoded[12..]).unwrap();
    Ok(decrypted)
}

impl Display for SensitiveData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let encrypted = encrypt(self.0.as_bytes()).unwrap();
        write!(f, "{}", encrypted)
    }
}

impl FromStr for SensitiveData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let decrypted = decrypt(s)?;
        let decrypted = String::from_utf8(decrypted)?;
        Ok(SensitiveData(decrypted))
    }
}

impl SensitiveData {
    fn new(data: impl Into<String>) -> Self {
        Self(data.into())
    }
}
