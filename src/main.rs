use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Side {
    Bid, Offer
}

#[derive(Debug)]
pub struct OrderEntry {
    pub side: Side,
    pub price: i64,
    pub amount: i64
}

#[derive(Debug)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: BTreeMap<i64, OrderEntry>,
    pub offers: BTreeMap<i64, OrderEntry>
}

fn main() {
    println!("Hello, world!");
    let mut ob = OrderBook {
        symbol: "BTC/USDT".to_string(),
        bids: BTreeMap::new(),
        offers: BTreeMap::new()
    };

    let bid = OrderEntry {
        side: Side::Bid,
        price: 40367,
        amount: 1
    };

    let offer = OrderEntry {
        side: Side::Offer,
        price: 40361,
        amount: 2
    };

    ob.bids.insert(bid.amount, bid);
    ob.offers.insert(offer.amount, offer);

    println!("{:?}", ob);
}
