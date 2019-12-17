use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use std::path::{Path, PathBuf};

fn index() -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("./public/index.html"))?)
}

fn favicon() -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("./public/static/favicon.ico"))?)
}

fn files(req: HttpRequest) -> Result<NamedFile> {
    let file: PathBuf = req.match_info().query("filename").parse().unwrap();
    if let Ok(file) = NamedFile::open(Path::new("./public").join(&file)) {
        Ok(file)
    } else {
        Ok(NamedFile::open(Path::new("./dist").join(file))?)
    }
}

fn main() {
    println!("The server has been started!: http://0.0.0.0:7878 \n");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/favicon.ico", web::get().to(favicon))
            .route("/{filename:.*}", web::get().to(files))
    })
    .bind("0.0.0.0:7878")
    .unwrap()
    .run()
    .unwrap();
}
