use tide_rhai::RhaiDir;
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/*")
        .get(RhaiDir::new("/*", "./examples/app/").unwrap());
    log::info!("Visit samples:");
    log::info!("http://127.0.0.1:8080/helloworld.rhai:");
    log::info!("http://127.0.0.1:8080/headers.rhai");
    log::info!("http://127.0.0.1:8080/fetch.rhai");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
