const API_URL: &str = env!("api_url");

lazy_static::lazy_static! {
    pub static ref CLIENT: client::Client = client::Client::new(API_URL.to_owned()).unwrap();
}
