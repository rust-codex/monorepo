use lib_utils::envs::get_env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    lib_log::init();

    let a = get_env("TESTEE").unwrap();
    info!("{}", a);
    Ok(())
}
