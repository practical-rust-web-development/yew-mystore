use actix_files::NamedFile;
use actix_web::{middleware, web, App, Error, HttpServer};

const ASSETS_DIR: &str = "../static";

async fn serve_index_html() -> Result<NamedFile, Error> {
    const INDEX_HTML: &str = "index.html";
    let index_file = format!("{}/{}", ASSETS_DIR, INDEX_HTML);

    Ok(NamedFile::open(index_file)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let localhost: &str = "0.0.0.0";
    let port: u16 = 8000;
    let addr = (localhost, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/", ASSETS_DIR).index_file("index.html"))
            .default_service(web::get().to(serve_index_html))
    })
    .bind(addr)?
    .workers(1)
    .run()
    .await
}