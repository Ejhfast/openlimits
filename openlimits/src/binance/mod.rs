use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use shared::Result;

use crate::exchange::Exchange;
use crate::model::{Asks, Bids, OrderBookRequest, OrderBookResponse};

#[derive(Deref, DerefMut)]
pub struct Binance(binance::Binance);

impl Binance {
    pub fn new(sandbox: bool) -> Self {
        Binance(binance::Binance::new(sandbox))
    }

    pub fn with_credential(api_key: &str, api_secret: &str, sandbox: bool) -> Self {
        Binance(binance::Binance::with_credential(
            api_key, api_secret, sandbox,
        ))
    }
}

#[async_trait]
impl Exchange for Binance {
    async fn order_book(&mut self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.get_depth(req.symbol.as_str(), None)
            .await
            .map(Into::into)
    }
}

impl From<binance::model::OrderBook> for OrderBookResponse {
    fn from(book: binance::model::OrderBook) -> Self {
        Self {
            last_update_id: book.last_update_id,
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<binance::model::Bids> for Bids {
    fn from(bids: binance::model::Bids) -> Self {
        Self {
            price: bids.price,
            qty: bids.qty,
        }
    }
}

impl From<binance::model::Asks> for Asks {
    fn from(bids: binance::model::Asks) -> Self {
        Self {
            price: bids.price,
            qty: bids.qty,
        }
    }
}
