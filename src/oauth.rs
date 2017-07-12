use chrono;
use serde_qs as qs;
use std::io;
use rand::{self, Rng};



/*
Client(
    client_key=consumer_key,
    resource_owner_key=consumer_key,
    signature_method=SIGNATURE_RSA,
    signature_type=SIGNATURE_TYPE_AUTH_HEADER,
    rsa_key=rsa_key,

    client_secret=None,
    resource_owner_secret=None,
    callback_uri=None,
    verifier=None,
    realm=None, nonce=None, timestamp=None)
*/

const SIGNATURE_HMAC: &'static str = "HMAC-SHA1";
const SIGNATURE_RSA: &'static str = "RSA-SHA1";

#[derive(Serialize)]
struct Params {
    oauth_consumer_key: String,
    oauth_nonce: String,
    oauth_signature_method: &'static str,
    oauth_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_token: Option<String>,
    #[serde(skip_serializing)]
    oauth_token_secret: Option<String>,
    oauth_version: &'static str,
}

impl Params {
    fn new<Str: Into<String>>(consumer_key: Str, signature_method: &'static str) -> Result<Params, io::Error> {
        Ok(Params{
            oauth_consumer_key: consumer_key.into(),
            oauth_nonce: generate_nonce()?, // FIXME: Don't unwrap
            oauth_signature_method: signature_method,
            oauth_timestamp: generate_timestamp(),
            oauth_token: None,
            oauth_token_secret: None,
            oauth_version: "1.0",
        })
    }

    fn signature_base(method: &str, base_url: &str, params: &str) -> Result<String, qs::ser::Error> {
        Ok(format!("{}&{}&{}", method.to_uppercase(), qs::to_string(&base_url)?, qs::to_string(&params)?))
    }

    fn signing_key(consumer_secret: &str, token_secret: &str) -> String {
        format!("{}&{}", consumer_secret, token_secret)
    }
}

fn generate_nonce() -> Result<String, io::Error> {
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

fn header(consumer_key: &str, nonce: &str, signature: &str, signature_method: &'static str, timestamp: &str) -> String {
    format!("
    OAuth oauth_consumer_key=\"{}\",
          oauth_nonce=\"{}\",
          oauth_signature=\"{}\",
          oauth_signature_method=\"{}\",
          oauth_timestamp=\"{}\",
          oauth_version=\"1.0\"",
          consumer_key,
          nonce,
          signature,
          signature_method,
          timestamp)
}
