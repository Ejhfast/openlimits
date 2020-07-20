use serde::{Deserialize, Serialize};
use shared::string_to_float;
pub mod websocket;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub display_name: String,
    pub quote_currency: String,
    pub base_currency: String,
    #[serde(with = "string_to_float")]
    pub base_increment: f64,
    #[serde(with = "string_to_float")]
    pub quote_increment: f64,
    #[serde(with = "string_to_float")]
    pub base_min_size: f64,
    #[serde(with = "string_to_float")]
    pub base_max_size: f64,
    pub min_market_funds: String,
    pub max_market_funds: String,
    pub status: String,
    pub status_message: String,
    pub cancel_only: bool,
    pub limit_only: bool,
    pub post_only: bool,
    pub trading_disabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    #[serde(with = "string_to_float")]
    pub balance: f64,
    #[serde(with = "string_to_float")]
    pub available: f64,
    #[serde(with = "string_to_float")]
    pub hold: f64,
    pub profile_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candle {
    pub time: i64,
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub trade_id: i64,
    pub time: String,
    pub size: String,
    #[serde(with = "string_to_float")]
    pub price: f64,
    pub side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ticker {
    pub trade_id: i64,
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub size: f64,
    #[serde(with = "string_to_float")]
    pub bid: f64,
    #[serde(with = "string_to_float")]
    pub ask: f64,
    #[serde(with = "string_to_float")]
    pub volume: f64,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book<T> {
    pub sequence: usize,
    pub bids: Vec<T>,
    pub asks: Vec<T>,
}

pub trait BookLevel {
    fn level() -> u8;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL1 {
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub size: f64,
    pub num_orders: usize,
}

impl BookLevel for BookRecordL1 {
    fn level() -> u8 {
        1
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL2 {
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub size: f64,
    pub num_orders: usize,
}

impl BookLevel for BookRecordL2 {
    fn level() -> u8 {
        2
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL3 {
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub size: f64,
    pub order_id: String,
}

impl BookLevel for BookRecordL3 {
    fn level() -> u8 {
        3
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: String,
    pub product_id: String,
    pub side: OrderSide,
    pub stp: Option<String>,
    #[serde(flatten)]
    pub _type: OrderType,
    pub post_only: bool,
    pub created_at: String,
    #[serde(with = "string_to_float")]
    pub fill_fees: f64,
    #[serde(with = "string_to_float")]
    pub filled_size: f64,
    #[serde(with = "string_to_float")]
    pub executed_value: f64,
    pub status: OrderStatus,
    pub settled: bool,
    #[serde(flatten)]
    pub stop: Option<OrderStop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderRequest {
    pub side: OrderSide,
    pub client_oid: Option<String>,
    pub product_id: String,
    #[serde(flatten)]
    pub _type: OrderRequestType,
    #[serde(flatten)]
    pub stop: Option<OrderStop>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OrderType {
    Limit {
        #[serde(with = "string_to_float")]
        size: f64,
        #[serde(with = "string_to_float")]
        price: f64,
        #[serde(flatten)]
        time_in_force: OrderTimeInForce,
    },
    Market {
        #[serde(default)]
        #[serde(with = "string_to_float")]
        size: f64,
        #[serde(default)]
        #[serde(with = "string_to_float")]
        funds: f64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum OrderRequestType {
    Limit {
        price: f64,
        size: f64,
        post_only: bool,
        #[serde(flatten)]
        time_in_force: Option<OrderTimeInForce>,
    },
    Market {
        #[serde(flatten)]
        _type: OrderRequestMarketType,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum OrderRequestMarketType {
    Size { size: f64 },
    Funds { funds: f64 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "time_in_force")]
pub enum OrderTimeInForce {
    GTC,
    GTT { expire_time: String },
    IOC,
    FOK,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    Open,
    Done,
    Pending,
    Active,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStop {
    stop_price: f64,
    #[serde(rename = "stop")]
    _type: OrderStopType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderStopType {
    Loss,
    Entry,
}
