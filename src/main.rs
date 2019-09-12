#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/<file..>", rank = 0)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("dist/").join(file)).ok()
}

#[get("/styles.css")]
fn styles() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/styles.css")).ok()
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files, styles, favicon])
        .launch();
}
