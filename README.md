# Mempool Arbitrage MEV Bot

> **AVISO EDUCATIVO** — Este repositório documenta conceitos de MEV para aprendizado. Executar front-running/sandwich em mainnet pode ser antiético, ilegal em algumas jurisdições, e competir com searchers profissionais é economicamente inviável para a maioria.

## Stack

- Rust
- ethers-rs

## O que o bot faz (conceitualmente)

1. Subscreve mempool via `eth_subscribe("pendingTransactions")`.
2. Decodifica swaps Uniswap V2/V3 pendentes.
3. Simula profit após gas.
4. Envia bundle via Flashbots (se lucrativo).

## Infraestrutura de rede para latência

| Componente | Requisito |
|------------|-----------|
| Nó Ethereum | Erigon/Geth archive, SSD NVMe |
| Conexão | < 5 ms ao relay (co-location) |
| Peers | Máximo de peers de baixa latência |

Sem co-location, você perde para searchers com latência sub-ms.

## Limitações do gas fee

```
profit = amount_out_simulated - amount_in - gas_cost
```

Se `profit <= 0`, descarte. Em congestão, gas price sobe e elimina oportunidades marginais.

## Estrutura

| Pasta | Função |
|-------|--------|
| `src/scanner/` | Mempool listener |
| `src/executor/` | TX builder |
| `src/strategy/` | Arbitragem / sandwich |

## Build

```bash
cargo build --release
```

Configure `config/network.toml` com RPC e chave **apenas em testnet**.

## Leitura recomendada

- [docs/MEV_CONCEPTS.md](docs/MEV_CONCEPTS.md)
- Flashbots docs
