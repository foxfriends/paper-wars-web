#[macro_use]
extern crate rocket;

use std::path::PathBuf;
use rocket::response::NamedFile;
use rocket::Request;
use rocket_contrib::serve::{Options, StaticFiles};

const STATIC_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

#[catch(404)]
async fn index(_req: &Request<'_>) -> NamedFile {
    NamedFile::open(format!("{}/index.html", STATIC_DIR)).await.unwrap()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let static_files = StaticFiles::new(STATIC_DIR, Options::Index | Options::NormalizeDirs);
    rocket::ignite()
        .mount("/", static_files)
        .register(catchers![index])
        .launch()
        .await
        .unwrap();
}
