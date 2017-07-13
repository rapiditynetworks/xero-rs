extern crate xero;

use std::env;
use xero::accounting;

fn main() {
    let consumer_key = env::var("XERO_CONSUMER_KEY").expect("Environment XERO_CONSUMER_KEY must be provided");
    let private_key = env::var("RSA_PRIVATE_KEY_PEM").expect("Environment RSA_PRIVATE_KEY_PEM must be provided");
    let keypair = xero::PKey::from_rsa(xero::Rsa::private_key_from_pem(private_key.as_bytes()).unwrap()).unwrap();
    let client = xero::Client::new(xero::PrivateApplication::new(consumer_key, keypair).unwrap());

    // Get all contacts
    let contacts = accounting::Contacts::get(&client);
    println!("{:?}", contacts);

    // Create one contact
    let mut params = accounting::ContactParams::default();
    params.name = Some("Just an Example Company");
    let contact = accounting::Contact::put(&client, params);
    println!("{:?}", contact);
}
