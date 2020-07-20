use openlimits::binance::Binance;
use openlimits::exchange::Exchange;
use openlimits::model::OrderBookRequest;

#[tokio::test]
async fn order_book() {
    let exchange = Binance::new(true);
    let req = OrderBookRequest{symbol: "BNBBTC".to_string()};
    let resp = exchange.order_book(&req).await.unwrap();
    println!("{:?}", resp);
}
