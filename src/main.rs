#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;

mod decimal;
mod model;
mod orderbook;
mod websocket_client;

use crate::decimal::Decimal;
use crate::model::Side::{Bid, Offer};
use crate::orderbook::{OrderBook, OrderEntry};
use actix::prelude::*;
use ws::{CloseCode, Handler, Handshake, Sender};

struct WsClient {
    out: Sender,
}

impl Handler for WsClient {
    fn on_open(&mut self, _shake: Handshake) -> ws::Result<()> {
        println!("Connection open!");
        let subscribe_msg = r#"{"op": "subscribe", "args": ["orderBookL2_25:XBTUSD"]}"#;
        self.out.send(subscribe_msg)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("{}", msg.as_text().unwrap());
        ws::Result::Ok(())
    }

    fn on_close(&mut self, _code: CloseCode, reason: &str) {
        format!("Connection closed! {}", reason);
    }
}

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor started!");
        ws::connect("wss://ws.bitmex.com/realtime", |out| { WsClient { out }}).unwrap()
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct Ping;

fn main() {
/*    println!("Hello, world!");

    let dec = Decimal::new(0.2 * 0.2);
    println!("0.2 * 0.2 = {}", dec);

    let mut ob = OrderBook::new("BTC/USDT".into());

    let bid1 = OrderEntry::new(Bid, 40355.4.into(), 1.2.into());
    let bid2 = OrderEntry::new(Bid, 40358.0.into(), 0.55.into());
    let offer = OrderEntry::new(Offer, 40361.4.into(), 0.4.into());

    ob.insert(bid1);
    ob.insert(bid2);
    ob.insert(offer);

    println!("{:?}", ob);
*/
    // starting Actix actor

    // let system = actix::System::new();
    // system.run();

    // let _address = MyActor.start();

    // let mut system = System::new();
    // let _addr = system.block_on(async { MyActor.start() });
    // system.run();

    ws::connect("wss://ws.bitmex.com/realtime", |out| { WsClient { out }}).unwrap();

}
