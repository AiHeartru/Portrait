extern crate rocket;

use std::error::Error;
use std::path::Path;

use rocket::fs::{FileServer, NamedFile, relative};
use rocket::{get, routes};
use base::*;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build().mount("/", routes![index]).mount("/", FileServer::from("static/")).launch().await?;
    Ok(())
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    let pic = Base::gen_portrait();
    let mut path = Path::new(relative!("../static")).to_path_buf();
    if path.is_dir() {
        path.push(pic)
    }
    NamedFile::open(path).await.ok()
}