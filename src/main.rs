#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;

mod decimal;
mod model;

use crate::decimal::Decimal;
use crate::model::{OrderBook, OrderEntry, Side};
use crate::Side::{Bid, Offer};

fn main() {
    println!("Hello, world!");

    let dec = Decimal::new(0.2 * 0.2);
    println!("0.2 * 0.2 = {}", dec);

    let mut ob = OrderBook::new("BTC/USDT".into());

    let bid1= OrderEntry::new(Bid, 40355.4.into(), 1.2.into());
    let bid2 = OrderEntry::new(Bid, 40358.0.into(), 0.55.into());
    let offer = OrderEntry::new(Offer, 40361.4.into(), 0.4.into());

    ob.insert(bid1);
    ob.insert(bid2);
    ob.insert(offer);

    println!("{:?}", ob);
}
