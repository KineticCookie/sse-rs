use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use futures::Future;
use serde_json::{Value, Error};
use serde::{Serialize, Deserialize};
use hyper::client::*;
use hyper::{Uri, Post};
use tokio_core::reactor::Core;

use messages;

#[derive(Debug)]
pub struct EngineClient {
    core: Core,
    client: Client<HttpConnector>,
    pub addr: String,
}

impl EngineClient {
    pub fn new(addr: &str) -> EngineClient {
        let core = Core::new().unwrap();
        let client = Client::new(&core.handle());
        EngineClient {
            core: core,
            client: client,
            addr: addr.to_owned(),
        }
    }
    pub fn detect_server() -> Result<EngineClient, Error> {
        let path = "C:\\ProgramData\\SteelSeries\\SteelSeries Engine 3\\coreProps.json";
        let mut file = File::open(path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer);
        let json: Value = ::serde_json::from_str(&buffer).unwrap();
        Ok(EngineClient::new(json["address"].as_str().unwrap()))
    }

    pub fn send_msg<'a, T: messages::EngineMessage + Serialize + Deserialize<'a>>(&mut self, msg: &'a T) {
        let uri = format!("http://{0}/{1}", self.addr, msg.get_url());
        println!("{}", uri);
        let mut request = Request::new(Post, Uri::from_str(&uri).unwrap());
        request.set_body(::serde_json::to_string(&msg).unwrap());
        {
            let headers = request.headers_mut();
            headers.append_raw("Content-Type", "application/json");
        }
        println!("{:?}", request);
        let res = self.client.request(request).map(|res| {
            println!("Response: {}", res.status());
        });
        self.core.run(res);
    }
}
