use std::cmp::Ordering;
use crate::decimal::Decimal;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Side {
    Bid, Offer
}

#[derive(Debug, Copy, Clone)]
pub struct OrderKey {
    pub side: Side,
    pub price: Decimal
}

#[derive(Debug, Copy, Clone)]
pub struct OrderEntry {
    pub key: OrderKey,
    pub amount: Decimal
}

impl OrderEntry {
    pub fn new(side: Side, price: Decimal, amount: Decimal) -> OrderEntry {
        let key = OrderKey {side, price};
        return OrderEntry {key, amount};
    }
}

impl Ord for OrderKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let part = self.partial_cmp(other);
        match part {
            Some(res) => res,
            None => if self.side == Side::Bid { Ordering::Less } else { Ordering::Greater }
        }
    }
}

impl PartialOrd for OrderKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self.side == other.side {
            if self.side == Side::Offer {
                Some(self.price.cmp(&other.price))
            } else {
                Some(other.price.cmp(&self.price))
            }
        } else { None }
    }
}

impl PartialEq for OrderKey {
    fn eq(&self, other: &Self) -> bool {
        return self.side == other.side && self.price == other.price
    }
}

impl Eq for OrderKey {}

#[derive(Debug)]
pub struct OrderBook {
    pub symbol: String,
    bids: BTreeMap<OrderKey, OrderEntry>,
    offers: BTreeMap<OrderKey, OrderEntry>
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
        let line = if entry.key.side == Side::Bid { &mut self.bids } else {&mut self.offers};
        line.insert(entry.key, entry);
        return;
    }
}