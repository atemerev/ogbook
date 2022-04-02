use actix::io::SinkWrite;
use actix::prelude::*;
use actix_codec::Framed;
use awc::{error::WsProtocolError, ws, BoxedSocket, Client};
use futures::stream::{SplitSink, SplitStream};
use log::{error, info};
use openssl::ssl::SslConnector;

pub type WsFramedSink = SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>;
pub type WsFramedStream = SplitStream<Framed<BoxedSocket, ws::Codec>>;

pub struct WsClient {
    sink: SinkWrite<ws::Message, WsFramedSink>,
}

impl WsClient {
    pub fn start(sink: WsFramedSink, stream: WsFramedStream) -> Addr<Self> {
        WsClient::create(|ctx| {
            ctx.add_stream(stream);
            WsClient {
                sink: SinkWrite::new(sink, ctx),
            }
        })
    }
}

impl Actor for WsClient {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("WS client started");
    }
}

impl actix::io::WriteHandler<WsProtocolError> for WsClient {}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Event {
    pub payload: String,
}

impl Handler<Event> for WsClient {
    type Result = ();

    fn handle(&mut self, msg: Event, _ctx: &mut Self::Context) -> Self::Result {
        println!("message received");
        self.sink.write(ws::Message::Text(msg.payload.into()));
    }
}

impl StreamHandler<Result<ws::Frame, WsProtocolError>> for WsClient {
    fn handle(&mut self, item: Result<ws::Frame, WsProtocolError>, _ctx: &mut Self::Context) {
        use ws::Frame;
        match item.unwrap() {
            Frame::Text(text_bytes) => {
                let text = std::str::from_utf8(text_bytes.as_ref()).unwrap();
                println!("Receiving Message: {}", text);
            }
            Frame::Binary(_) => {}
            Frame::Continuation(_) => {}
            Frame::Ping(_) => {
                info!("Ping received!");
            }
            Frame::Pong(_) => {}
            Frame::Close(_) => {}
        }
    }
}


