use crate::decimal::Decimal;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
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
    bids: BTreeMap<Decimal, OrderEntry>,
    offers: BTreeMap<Decimal, OrderEntry>
}

impl OrderBook {
    pub fn new(symbol: String) -> Self {
        return OrderBook {
            symbol,
            bids: BTreeMap::new(),
            offers: BTreeMap::new()
        }
    }

    pub fn insert(&mut self, entry: OrderEntry) {
        let line = if entry.side == Side::Bid { &mut self.bids } else {&mut self.offers};
        line.insert(entry.price, entry);
        return;
    }
}