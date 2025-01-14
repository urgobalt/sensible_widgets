#![feature(let_chains, string_from_utf8_lossy_owned)]

use std::fmt::Debug;

use chunks_rs::position::{Edge, EdgeConfig, Layer};
use chunks_rs::utils::{load_css, tag_label};
use chunks_rs::widgets::Chunk;
use chunks_rs::{Factory, GtkApp, Internal};
use zbus::Connection;

use self::error::HandleDbusError;
use self::network::get_network;

mod error;
mod network;

const STYLE: &str = "
window {
    background-color: transparent;
}

#storage {
    font-size: 34px;
    background-color: #000000;
    color: #FFFFFF;
}
";

#[tokio::main]
async fn main() -> Result<(), error::MainError> {
    let connection = Connection::system().await.handling_system()?;
    println!("{:?}", get_network(&connection).await.unwrap_err());

    Ok(())

    // let factory = Factory::new("chunks.factory");
    //
    // let chunks = |factory: GtkApp| {
    //     load_css(STYLE);
    // };
    //
    // factory.pollute(chunks);
}
