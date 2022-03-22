#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;

mod decimal;
mod model;

use crate::decimal::Decimal;
use crate::model::{OrderBook, OrderEntry, Side};

fn main() {
    println!("Hello, world!");

    let dec = Decimal::new(0.2 * 0.2);
    println!("0.2 * 0.2 = {}", dec);

    let mut ob = OrderBook::new("BTC/USDT".into());

    let bid = OrderEntry {
        side: Side::Bid,
        price: 40367.0.into(),
        amount: 1.0.into()
    };

    let offer = OrderEntry {
        side: Side::Offer,
        price: 40361.4.into(),
        amount: 0.4.into()
    };

    ob.insert(bid);
    ob.insert(offer);

    println!("{:?}", ob);
}
