use rtre_market_data::{connect_to_market_data_feed, subscribe_to_ticker, handle_market_data};
use rtre_risk_calculator::calculate_var;
use tokio::runtime::Runtime;
use anyhow::Result;

pub fn run_risk_engine(url: &str, ticker: &str) -> Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let mut ws_stream = connect_to_market_data_feed(url).await.unwrap();
        subscribe_to_ticker(&mut ws_stream, ticker).await.unwrap();

        handle_market_data(ws_stream, |data| {
            if let Some(price) = data["p"].as_f64() {
                // Placeholder for volatility
                let volatility = 0.02;
                calculate_var(price, volatility, 0.95);
            }
        }).await;
    });
    Ok(())
}

