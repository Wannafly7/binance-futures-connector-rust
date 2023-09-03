use binance_futures_connector_rust::{
    hyper::{BinanceHttpClient, Error},
    market,
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let client = BinanceHttpClient::default();
    let request = market::ticker_twenty_four_hr()
        .symbol("BNBUSDT")
        .symbols(vec!["BTCUSDT", "BNBBTC"]);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
