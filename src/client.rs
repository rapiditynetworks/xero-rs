use application::Application;
use error::{Error, RequestError};
use hyper::Client as HttpClient;
use hyper::client::RequestBuilder;
use hyper::header::Headers;
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
        format!("https://api.xero.com/{}", &path[1..])
    }

    pub fn new<App: Application + 'static>(app: App) -> Client {
        let tls = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        let client = HttpClient::with_connector(connector);
        Client {client: client, application: Box::new(app)}
    }

    pub fn get<'a, T: serde::Deserialize>(&'a self, path: &'a str) -> Result<T, Error> {
        let url = Client::url(path);
        let headers = self.headers("GET", &url)?;
        let request = self.client.get(&url).headers(headers);
        send(request)
    }

    fn headers(&self, method: &str, url: &str) -> Result<Headers, Error> {
        let signature = self.application.get_signature(method, url)?;
        let mut headers = Headers::new();
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
