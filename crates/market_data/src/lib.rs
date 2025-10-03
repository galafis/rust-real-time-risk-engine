use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use url::Url;
use serde_json::Value;
use log::{info, error};
use anyhow::Result;
use futures_util::{StreamExt, SinkExt};

pub async fn connect_to_market_data_feed(url_str: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let url = Url::parse(url_str)?;
    let (ws_stream, _) = connect_async(url).await?;
    info!("Connected to market data feed.");
    Ok(ws_stream)
}

pub async fn subscribe_to_ticker(ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, ticker: &str) -> Result<()> {
    let subscribe_msg = serde_json::json!({ "type": "subscribe", "symbol": ticker });
    ws_stream.send(Message::Text(subscribe_msg.to_string())).await?;
    info!("Subscribed to ticker: {}", ticker);
    Ok(())
}

pub async fn handle_market_data<F>(mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, mut on_data: F)
where
    F: FnMut(Value),
{
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<Value>(&text) {
                    Ok(data) => on_data(data),
                    Err(e) => error!("Failed to parse JSON: {}", e),
                }
            }
            Ok(_) => {}
            Err(e) => error!("Error receiving message: {}", e),
        }
    }
}

