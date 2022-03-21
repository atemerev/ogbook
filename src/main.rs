#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;

mod decimal;

use std::collections::BTreeMap;
use crate::decimal::Decimal;

#[derive(Debug)]
pub enum Side {
    Bid, Offer
}

#[derive(Debug)]
pub struct OrderEntry {
    pub side: Side,
    pub price: Decimal,
    pub amount: Decimal
}

#[derive(Debug)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: BTreeMap<Decimal, OrderEntry>,
    pub offers: BTreeMap<Decimal, OrderEntry>
}

fn main() {
    println!("Hello, world!");

    let dec = Decimal::new(0.2 * 0.2);
    println!("0.2 * 0.2 = {}", dec);

    let mut ob = OrderBook {
        symbol: "BTC/USDT".to_string(),
        bids: BTreeMap::new(),
        offers: BTreeMap::new()
    };

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

    ob.bids.insert(bid.amount, bid);
    ob.offers.insert(offer.amount, offer);

    println!("{:?}", ob);
}
