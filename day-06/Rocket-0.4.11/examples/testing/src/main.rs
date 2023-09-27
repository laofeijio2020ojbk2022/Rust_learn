#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../index.html")).await.ok()
}
#[launch]
fn rocket() ->_{
    rocket::build().mount("/", routes![index])
}
