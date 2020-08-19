use client::Client;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT: Client = Client::new(env!("api_url").to_owned()).unwrap();
}
