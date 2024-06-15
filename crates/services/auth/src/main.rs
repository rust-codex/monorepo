use lib_log::hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", hello());
    Ok(())
}
