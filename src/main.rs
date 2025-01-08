use std::fmt::Debug;

use chunks_rs::position::{Edge, EdgeConfig, Layer};
use chunks_rs::utils::{load_css, tag_label};
use chunks_rs::widgets::Chunk;
use chunks_rs::{Factory, GtkApp, Internal};

mod dbus_access;
mod error;
mod network;
mod wpa_supplicant;

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

fn main() {
    let factory = Factory::new("chunks.factory");

    let chunks = |factory: GtkApp| {
        let right = right(&factory);

        load_css(STYLE);
    };

    factory.pollute(chunks);
}

fn right(factory: &GtkApp) {
    let tag = tag_label("right");
    let margins = vec![(Edge::Right, 20), (Edge::Top, 20)];
    let anchors = EdgeConfig::TOP_RIGHT.to_vec();

    let storage_closure = || {
        let network = Internal::get_network();
        if let Ok(network) = network {
            return format!("<span foreground='#FFFFFF'>{:.0}%</span>", network,);
        }
        println!("failed getting the network: {:?}", network.unwrap_err());
        String::new()
    };

    Internal::update_storage(&tag, storage_closure);

    Chunk::new(
        factory.clone(),
        "Network",
        tag,
        margins,
        anchors,
        Layer::Bottom,
        false, // change to true for tag_revealer
    )
    .build();
}
