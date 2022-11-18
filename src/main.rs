use zero_to_prod_rust::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
