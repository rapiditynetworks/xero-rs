#![allow(dead_code)]
use chrono;
use openssl;
use rand::{self, Rng};
use rustc_serialize::base64::{self, ToBase64};
use serde_qs as qs;
use std::{error, io, fmt};

pub const SIGNATURE_HMAC: &'static str = "HMAC-SHA1";
pub const SIGNATURE_RSA: &'static str = "RSA-SHA1";

#[derive(Serialize)]
pub struct Params {
    // IMPORTANT: Fields must be alphabetically sorted

    // TODO: Some sort of builder pattern or mode enumeration may be better

    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_callback: Option<String>,
    pub oauth_consumer_key: String,
    #[serde(skip_serializing)]
    pub oauth_consumer_secret: Option<String>,
    pub oauth_nonce: String,
    pub oauth_signature_method: &'static str,
    pub oauth_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    #[serde(skip_serializing)]
    pub oauth_token_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_verifier: Option<String>,
    pub oauth_version: &'static str,
}


impl Params {
    pub fn new<Str: Into<String>>(consumer_key: Str, signature_method: &'static str) -> Result<Params, Error> {
        Ok(Params{
            realm: None,
            oauth_callback: None,
            oauth_consumer_key: consumer_key.into(),
            oauth_consumer_secret: None,
            oauth_nonce: generate_nonce()?,
            oauth_signature_method: signature_method,
            oauth_timestamp: generate_timestamp(),
            oauth_token: None,
            oauth_token_secret: None,
            oauth_verifier: None,
            oauth_version: "1.0",
        })
    }

    // TODO: Handle url with query paramters
    pub fn sign_request(&self, keypair: &openssl::pkey::PKey, method: &str, base_url: &str) -> Result<String, Error> {
        let message = self.signature_base(method, base_url)?;
        let mut signer = openssl::sign::Signer::new(openssl::hash::MessageDigest::sha1(), keypair)?;
        signer.update(message.as_bytes())?;
        let signature_bytes = signer.finish()?;
        let signature_base64 = signature_bytes.to_base64(base64::STANDARD);

        Ok(format!("OAuth oauth_consumer_key=\"{}\", oauth_nonce=\"{}\", oauth_signature=\"{}\", oauth_signature_method=\"{}\", oauth_timestamp=\"{}\", oauth_version=\"1.0\"",
                   self.oauth_consumer_key, self.oauth_nonce, signature_base64, self.oauth_signature_method, self.oauth_timestamp))
    }

    fn signature_base(&self, method: &str, base_url: &str) -> Result<String, Error> {
        Ok(format!("{}&{}&{}", method.to_uppercase(), qs::to_string(&base_url)?, qs::to_string(self)?))
    }

    fn signing_key(&self) -> Result<String, Error> {
        let consumer_secret = self.oauth_consumer_secret.as_ref().map(|s| s.as_ref()).unwrap_or("");
        if self.oauth_signature_method == SIGNATURE_RSA {
            Ok(consumer_secret.to_string())
        } else {
            let token_secret = self.oauth_token_secret.as_ref().map(|s| s.as_ref()).unwrap_or("");
            Ok(format!("{}&{}", qs::to_string(&consumer_secret)?, qs::to_string(&token_secret)?))
        }
    }
}

fn generate_nonce() -> Result<String, Error> {
    let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::OsRng::new()?;
    let mut nonce = String::with_capacity(32);
    for _ in 0..32 {
        nonce.push(rng.choose(alphabet).unwrap().clone().into());
    }
    Ok(nonce)
}

fn generate_timestamp() -> String {
    chrono::UTC::now().timestamp().to_string()
}


#[derive(Debug)]
pub struct Error(Box<error::Error + Send>);

impl error::Error for Error {
    fn description(&self) -> &str { "oauth error" }
    fn cause(&self) -> Option<&error::Error> { Some(&*self.0) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))?;
        write!(f, ": {}", self.0)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error(Box::new(err))
    }
}

impl From<qs::ser::Error> for Error {
    fn from(err: qs::ser::Error) -> Error {
        Error(Box::new(err))
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Error {
        Error(Box::new(err))
    }
}
