use error::{Error, RequestError};
use hyper::Client as HttpClient;
use hyper::client::RequestBuilder;
use hyper::net::HttpsConnector;
use serde;
use serde_json as json;
use std::collections::HashMap;
use std::io::Read;

pub enum Credentials {
    Private(PrivateCredentials)
}

impl Credentials {
    fn private<Str: Into<String>>(consumer_key: Str, rsa_key: Str) -> Credentials {
        Credentials::Private(PrivateCredentials{})
    }
}

struct PrivateCredentials {

}

pub struct Client {
    client: HttpClient,
    creds: Credentials,
}

impl Client {
    fn url(path: &str) -> String {
        format!("https://api.xero.com/{}", &path[1..])
    }

    pub fn new(creds: Credentials) -> Client {
        use hyper_native_tls::NativeTlsClient;

        let tls = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        let client = HttpClient::with_connector(connector);
        Client {client: client, creds: creds}
    }

    pub fn get<'a, T: serde::Deserialize>(&'a self, path: &'a str) -> Result<T, Error> {
        let url = Client::url(path);
        let request = self.client.get(&url);
        send(request)
    }
}

fn send<T: serde::Deserialize>(request: RequestBuilder) -> Result<T, Error> {
    let mut response = request.send()?;
    let mut body = String::with_capacity(4096);
    response.read_to_string(&mut body)?;
    let status = response.status_raw().0;
    match status {
        200...299 => {}
        _ => { return Err(Error::from(RequestError{})); }
    }

    json::from_str(&body).map_err(|err| Error::from(err))
}
