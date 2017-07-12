use error::Error;
use oauth;
use openssl;

pub trait Application {
    fn get_signature(&self, url: &str, verb: &str) -> Result<String, Error>;
}

pub struct PrivateApplication {
    oauth: oauth::Params,
    keypair: openssl::pkey::PKey,
}

impl PrivateApplication {
    pub fn new<Str: Into<String>>(consumer_key: Str, rsa_keypair: openssl::pkey::PKey) -> Result<Self, Error> {
        let key = consumer_key.into();
        let mut params = oauth::Params::new(key.clone(), oauth::SIGNATURE_RSA)?;
        params.oauth_token = Some(key);
        Ok(PrivateApplication{oauth: params, keypair: rsa_keypair})
    }
}

impl Application for PrivateApplication {
    fn get_signature(&self, uri: &str, verb: &str) -> Result<String, Error> {
        Ok(self.oauth.sign_request(&self.keypair, uri, verb)?)
    }
}
