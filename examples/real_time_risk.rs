use rtre_core::run_risk_engine;
use rtre_utils::setup_logger;
use anyhow::Result;

fn main() -> Result<()> {
    setup_logger();

    // Using a public test WebSocket feed
    let url = "wss://ws.finnhub.io?token=c1mrj2aad3i9vj8jpsn0";
    let ticker = "AAPL";

    run_risk_engine(url, ticker)?;

    Ok(())
}

