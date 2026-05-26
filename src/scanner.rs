use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct PendingTx {
    pub hash: String,
    pub to: Option<String>,
    pub value_wei: u128,
}

pub async fn subscribe_pending() -> anyhow::Result<mpsc::Receiver<PendingTx>> {
    let (tx, rx) = mpsc::channel(1024);
    // Simula mempool em modo scaffold
    tokio::spawn(async move {
        for i in 0..5 {
            let _ = tx
                .send(PendingTx {
                    hash: format!("0x{:064x}", i),
                    to: Some("0xUniswapRouter".into()),
                    value_wei: 1_000_000,
                })
                .await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
    Ok(rx)
}
