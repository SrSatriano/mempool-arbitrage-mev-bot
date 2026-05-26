mod scanner;
mod strategy;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("MEV Bot — EDUCATIONAL / TESTNET ONLY");
    let rx = scanner::subscribe_pending().await?;
    while let Some(tx) = rx.recv().await {
        if let Some(opp) = strategy::evaluate(&tx) {
            println!("[sim] opportunity profit_wei={}", opp.estimated_profit_wei);
        }
    }
    Ok(())
}
