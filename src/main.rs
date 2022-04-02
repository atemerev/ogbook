#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;

mod decimal;
mod model;
mod orderbook;
mod websocket_client;

use futures::StreamExt;
use crate::decimal::Decimal;
use crate::model::Side::{Bid, Offer};
use crate::orderbook::{OrderBook, OrderEntry};
use openssl::ssl::SslConnector;
use crate::websocket_client::{WsFramedSink, WsFramedStream, WsClient, Event};

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ob = OrderBook::new("BTC/USDT".into());

    let bid1 = OrderEntry::new(Bid, 40355.4.into(), 1.2.into());
    let bid2 = OrderEntry::new(Bid, 40358.0.into(), 0.55.into());
    let offer = OrderEntry::new(Offer, 40361.4.into(), 0.4.into());

    ob.insert(bid1);
    ob.insert(bid2);
    ob.insert(offer);

    println!("{:?}", ob);

    std::env::set_var("RUST_LOG", "info");
    // env_logger::init();
    let ssl = {
        let mut ssl = SslConnector::builder(openssl::ssl::SslMethod::tls()).unwrap();
        let _ = ssl.set_alpn_protos(b"\x08http/1.1");
        ssl.build()
    };
    let connector = awc::Connector::new().openssl(ssl);
    let (_, framed) = awc::Client::new()
        .ws("wss://ws.bitmex.com/realtime")
        .connect()
        .await?;
    let (sink, stream): (WsFramedSink, WsFramedStream) = framed.split();
    let addr = WsClient::start(sink, stream);

    let _res = addr
        .send(Event {
            payload: r#"{"op": "subscribe", "args": ["orderBookL2_25:XBTUSD"]}"#.into()
        })
        .await
        .unwrap();
    let _ = actix_rt::signal::ctrl_c().await?;
    Ok(())

    // starting Actix actor

    // let system = actix::System::new();
    // system.run();

    // let _address = MyActor.start();

    // let mut system = System::new();
    // let _addr = system.block_on(async { MyActor.start() });
    // system.run();

    // ws::connect("wss://ws.bitmex.com/realtime", |out| { WsClient { out }}).unwrap();

}
