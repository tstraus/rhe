#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Json;

// struct ser
#[derive(Serialize, Deserialize)]
struct Response {
    username: String,
    #[serde(rename = "clientID")]
    client_id: u64
}

// get static root file
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

// server static files
#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

// request parameters
#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello {}", name)
}

// json response
#[get("/json")]
fn json() -> Json<Response> {
    Json(Response {
        username: "asdf".to_string(),
        client_id: 1234
    })
}

// start the server
fn main() {
    rocket::ignite().mount("/", routes![index, files, hello, json]).launch();
}