#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate futures;
extern crate tokio_core;

use hyper::*;
use std::str::FromStr;
use futures::future::Future;
use messages::EngineMessage;

mod messages;
mod client;

fn main() {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let client = Client::new(&core.handle());
    let mut server = client::EngineClient::detect_server().unwrap();
    println!("SteelSeries engine is detected @ {}", server.addr);
    let reg = messages::RegisterGame {
        game: "RUST_CLIENT".to_owned(),
        game_display_name: "Rusty client for SSE".to_owned(),
        icon_color_id: 4,
    };
    server.send_msg(&reg);
}
