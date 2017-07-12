use application::Application;
use error::{Error, RequestError};
use hyper::Client as HttpClient;
use hyper::client::RequestBuilder;
use hyper::header::{Accept, ContentType, Headers};
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use serde;
use serde_json as json;
use std::io::Read;

// TODO: Probably use trait

pub struct Client {
    client: HttpClient,
    application: Box<Application>
}

impl Client {
    fn url(path: &str) -> String {
        format!("https://api.xero.com/api.xro/2.0/{}", &path[1..])
    }

    pub fn new<App: Application + 'static>(app: App) -> Client {
        let tls = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        let client = HttpClient::with_connector(connector);
        Client {client: client, application: Box::new(app)}
    }

    pub fn get<'a, T: serde::Deserialize>(&'a self, path: &'a str) -> Result<T, Error> {
        let url = Client::url(path);
        let headers = self.headers("GET", &url, None)?;
        let request = self.client.get(&url).headers(headers);
        send(request)
    }

    pub fn put<T: serde::Deserialize>(&self, path: &str, body: &[u8]) -> Result<T, Error> {
        let url = Client::url(path);
        let headers = self.headers("PUT", &url, Some(body))?;
        let request = self.client.post(&url).headers(headers).body(body);
        send(request)
    }

    fn headers(&self, method: &str, url: &str, body: Option<&[u8]>) -> Result<Headers, Error> {
        // TODO: Add body to signature
        let signature = self.application.get_signature(method, url)?;
        let mut headers = Headers::new();
        headers.set(Accept::json());
        if method != "GET" {
            headers.set(ContentType::form_url_encoded());
        }
        headers.set_raw("Authorization", vec![signature.as_bytes().to_vec()]);
        Ok(headers)
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
