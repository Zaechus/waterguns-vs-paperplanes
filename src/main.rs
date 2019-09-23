#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/index.html")).ok()
}

#[catch(404)]
fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/404.html")).ok()
}

#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("dist/").join(file)).ok()
}

#[get("/static/<file..>", rank = 1)]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/static/").join(file)).ok()
}

#[get("/favicon.ico", rank = 0)]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/static/favicon.ico")).ok()
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index, files, static_files, favicon])
        .launch();
}
