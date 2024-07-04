use lib_log::hello;
use lib_utils::envs::get_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = get_env("TESTEE").unwrap();
    println!("{}", a);
    Ok(())
}
