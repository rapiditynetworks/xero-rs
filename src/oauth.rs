#![allow(dead_code)]
use chrono;
use openssl;
use rand::{self, Rng};
use rustc_serialize::base64::{self, ToBase64};
use serde_urlencoded as urlencoded;
use std::{error, io, fmt};

mod percent {
    use percent_encoding::{utf8_percent_encode, SIMPLE_ENCODE_SET};
    define_encode_set! {
        // All non alphanumeric characters on the (US) keyboard, except '~', '-', '_', and '.'
        pub PERCENT_ENCODE_SET = [SIMPLE_ENCODE_SET]
            | {'`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '+', '=', '[', ']', '{', '}', '|', '\\', ';', ':', '\'', '"', ',', '<', '>', '/', '?'}
    }

    pub fn to_string(value: &str) -> String {
        utf8_percent_encode(value, PERCENT_ENCODE_SET).to_string()
    }
}

pub const SIGNATURE_HMAC: &'static str = "HMAC-SHA1";
pub const SIGNATURE_RSA: &'static str = "RSA-SHA1";

#[derive(Serialize)]
pub struct Params {
    // TODO: Some sort of builder pattern or mode enumeration may be better

    #[serde(skip_serializing)]
    pub realm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_callback: Option<String>,
    pub oauth_consumer_key: String,
    #[serde(skip_serializing)]
    pub oauth_consumer_secret: Option<String>,
    pub oauth_signature_method: &'static str,
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
            oauth_signature_method: signature_method,
            oauth_token: None,
            oauth_token_secret: None,
            oauth_verifier: None,
            oauth_version: "1.0",
        })
    }

    // TODO: Handle url with query paramters
    pub fn sign_request(&self, keypair: &openssl::pkey::PKey, method: &str, base_url: &str) -> Result<String, Error> {
        let signature: String;
        let nonce = generate_nonce()?;
        let timestamp = generate_timestamp();
        let mut params = self.get_oauth_params(&nonce, &timestamp);

        let message = self.get_signature_base(method, base_url, &urlencoded::to_string(&params)?)?;
        let mut signer = openssl::sign::Signer::new(openssl::hash::MessageDigest::sha1(), keypair)?;
        signer.update(message.as_bytes())?;
        let signature_bytes = signer.finish()?;
        signature = percent::to_string(&signature_bytes.to_base64(base64::STANDARD));
        params.push(("oauth_signature", &signature));

        let formatted: Vec<String> = params.iter().map(|p| format!("{}=\"{}\"", p.0, p.1)).collect();
        let header = formatted.join(", ");
        Ok(format!("OAuth {}", header))
    }

    fn get_oauth_params<'a>(&'a self, nonce: &'a str, timestamp: &'a str) -> Vec<(&'a str, &'a str)> {
        let mut params: Vec<(&'a str, &'a str)> = Vec::new();
        if let Some(ref callback) = self.oauth_callback { params.push(("oauth_callback", callback)); }
        params.push(("oauth_consumer_key", &self.oauth_consumer_key));
        params.push(("oauth_nonce", nonce));
        params.push(("oauth_signature_method", self.oauth_signature_method));
        params.push(("oauth_timestamp", timestamp));
        if let Some(ref token) = self.oauth_token { params.push(("oauth_token", token)); }
        if let Some(ref verifier) = self.oauth_verifier { params.push(("oauth_verifier", verifier)); }
        params.push(("oauth_version", self.oauth_version));
        params
    }

    fn get_signature_base(&self, method: &str, base_url: &str, params: &str) -> Result<String, Error> {
        Ok(format!("{}&{}&{}", method.to_uppercase(), percent::to_string(base_url), percent::to_string(params)))
    }

    fn get_signing_key(&self) -> Result<String, Error> {
        let consumer_secret = self.oauth_consumer_secret.as_ref().map(|s| s.as_ref()).unwrap_or("");
        if self.oauth_signature_method == SIGNATURE_RSA {
            Ok(consumer_secret.to_string())
        } else {
            let token_secret = self.oauth_token_secret.as_ref().map(|s| s.as_ref()).unwrap_or("");
            Ok(format!("{}&{}", percent::to_string(consumer_secret), percent::to_string(token_secret)))
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

impl From<urlencoded::ser::Error> for Error {
    fn from(err: urlencoded::ser::Error) -> Error {
        Error(Box::new(err))
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Error {
        Error(Box::new(err))
    }
}
