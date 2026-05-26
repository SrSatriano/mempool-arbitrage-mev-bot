use crate::scanner::PendingTx;

pub struct Opportunity {
    pub estimated_profit_wei: u128,
}

pub fn evaluate(tx: &PendingTx) -> Option<Opportunity> {
    if tx.to.as_deref()? == "0xUniswapRouter" && tx.value_wei > 0 {
        return Some(Opportunity {
            estimated_profit_wei: 1000,
        });
    }
    None
}
