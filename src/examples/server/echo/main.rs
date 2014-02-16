//! A WebSocket Server

#[crate_id = "echo"];

extern mod extra;
extern mod http;
extern mod ws;

use http::server::{Config, Server, Request, ResponseWriter};
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use http::headers::content_type::MediaType;
use extra::time;
use ws::server::WebSocketServer;

#[deriving(Clone)]
struct EchoServer;

impl Server for EchoServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
        w.headers.date = Some(time::now_utc());
        w.headers.content_type = Some(MediaType {
            type_: ~"text",
            subtype: ~"html",
            parameters: ~[(~"charset", ~"UTF-8")]
        });
        w.headers.server = Some(~"EchoServer");

        w.write(bytes!("<h1>Echo Server</h1>"));
    }
}

impl WebSocketServer for EchoServer {
    fn handle_ws_connect(&self, receiver: Port<~str>, sender: Chan<~str>) {
        spawn(proc() {
            loop {
                let payload = "Echo: " + receiver.recv();
                sender.send(payload);
            }
        });
    }
}

fn main() {
    EchoServer.ws_serve_forever();
}
