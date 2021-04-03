use tide_rhai::RhaiDir;
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/*")
        .get(RhaiDir::new("/*", "./examples/app/").unwrap());
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
