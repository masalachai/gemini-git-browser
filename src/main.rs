use std::{env, time::Duration};

use northstar::{Server, GEMINI_PORT};
use tokio;

use gemini_git_browser::actions;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    env_logger::init();

    let gemini_port = match env::var("GEMINI_PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => GEMINI_PORT
    };

    Server::bind(("0.0.0.0", gemini_port))
        .add_route("/robots.txt", actions::robots)
        .add_route("/favicon.txt", actions::favicon)
        .add_route("/index.gmi", actions::robots)
        .add_route("/", actions::index)
        .set_timeout(Duration::from_secs(10))
        .serve()
        .await
}
