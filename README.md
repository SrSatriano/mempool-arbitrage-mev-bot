<div align="center">

# Bot educacional de arbitragem MEV no mempool

**Bot educacional de arbitragem MEV no mempool**

<p>
  <a href="https://github.com/SrSatriano/mempool-arbitrage-mev-bot"><img src="https://img.shields.io/badge/GitHub-mempool-arbitrage-mev-bot-24292e?style=for-the-badge&logo=github&logoColor=white" alt="GitHub" /></a>
</p>

<p>
  <img src="https://img.shields.io/badge/versão-1.0.0-0ea5e9?style=flat-square" alt="versão" />
  <img src="https://img.shields.io/badge/licença-MIT-22c55e?style=flat-square" alt="licença" />
  <img src="https://img.shields.io/badge/idioma-pt--BR-blue?style=flat-square" alt="idioma" />
  <img src="https://img.shields.io/badge/CI-GitHub_Actions-8b5cf6?style=flat-square" alt="ci" />
</p>

<p><strong>Scanner de oportunidades MEV apenas para testnet e fins educacionais.</strong></p>

<p>
  Autor: <a href="https://github.com/SrSatriano">@SrSatriano</a> ·
  Release <strong>1.0.0</strong> (2026-03-26)
</p>

</div>

---

## Índice

1. [Visão geral](#visão-geral)
2. [Problema e solução](#problema-e-solução)
3. [Para quem é](#para-quem-é)
4. [Casos de uso](#casos-de-uso)
5. [Funcionalidades](#funcionalidades)
6. [Stack tecnológica](#stack-tecnológica)
7. [Arquitetura](#arquitetura)
8. [Estrutura do repositório](#estrutura-do-repositório)
9. [Pré-requisitos](#pré-requisitos)
10. [Instalação e execução](#instalação-e-execução)
11. [Configuração](#configuração)
12. [Testes](#testes)
13. [Performance](#performance)
14. [Deploy e operação](#deploy-e-operação)
15. [Limitações conhecidas](#limitações-conhecidas)
16. [Roadmap](#roadmap)
17. [Documentação complementar](#documentação-complementar)
18. [Segurança e licença](#segurança-e-licença)

---

## Visão geral

Este repositório faz parte do **portfólio de engenharia** mantido por [@SrSatriano](https://github.com/SrSatriano). A versão **1.0.0** entrega implementação do núcleo do produto, testes automatizados, pipeline de integração contínua e documentação operacional em **português brasileiro**.

O objetivo é permitir que você clone, execute e evolua o projeto com clareza — do desenvolvimento local ao deploy em produção.

## Problema e solução

| | |
|---|---|
| **Problema** | MEV é pouco compreendido sem ambiente seguro de experimentação. |
| **Solução** | Listener de pending txs, avaliação de lucro simulado e avisos éticos claros. |

## Para quem é

Estudantes de DeFi e desenvolvedores Rust.

## Casos de uso

- Laboratório em testnet
- Simulador local de mempool

## Funcionalidades

- [x] Listener de transações pendentes
- [x] Avaliação de profit teórico
- [x] Documentação ética e limitações
- [x] Configuração TOML
- [x] Simulador local

## Stack tecnológica

| Camada | Tecnologias |
|--------|-------------|
| **Principal** | Rust, Tokio, Web3 |

## Arquitetura

```mermaid
flowchart LR
  MEM[Mempool testnet] --> SCAN[Scanner Rust]
  SCAN --> SIM[Simulador profit]
```

Detalhamento de componentes, fluxos de dados e decisões de design: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Estrutura do repositório

| Caminho | Descrição |
|---------|-----------|
| `src/` | Código Rust |
| `config/` | TOML de rede |

## Pré-requisitos

Rust 1.75+. **Não use em mainnet com fundos reais.**

## Instalação e execução

```bash
git clone https://github.com/SrSatriano/mempool-arbitrage-mev-bot.git
cd mempool-arbitrage-mev-bot
```

```bash
cargo build --release
cargo run --release
```

## Configuração

| Variável | Descrição | Exemplo |
|----------|-----------|--------|
| `ETH_RPC_URL` | RPC testnet | `https://...` |

> **Importante:** nunca faça commit de arquivos `.env` com segredos reais. Use `.env.example` como referência.

## Testes

Execute a suíte de testes antes de abrir pull requests:

```bash
cargo test
```

A pipeline [`.github/workflows/ci.yml`](.github/workflows/ci.yml) repete build e testes em cada push para `main`.

## Performance

| Ambiente | Uso |
|----------|-----|
| Testnet | Simulação |

Metodologia, hardware de referência e flags de compilação: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Deploy e operação

| Guia | Conteúdo |
|------|----------|
| [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) | Homologação, produção e rollback |
| [docs/OPERATIONS.md](docs/OPERATIONS.md) | Monitoramento, alertas e incidentes |

## Limitações conhecidas

- Educativo; sem garantia de lucro

## Roadmap

- Dashboard de oportunidades históricas

## Documentação complementar

| Documento | Descrição |
|-----------|-----------|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Arquitetura e decisões técnicas |
| [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) | Deploy passo a passo |
| [docs/OPERATIONS.md](docs/OPERATIONS.md) | Runbook operacional |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Como contribuir |
| [CHANGELOG.md](CHANGELOG.md) | Histórico de versões |
| [SECURITY.md](SECURITY.md) | Política de segurança |
| [AUTHORS.md](AUTHORS.md) | Créditos |

## Segurança e licença

- Dependências revisadas na release **1.0.0**
- Vulnerabilidades: siga [SECURITY.md](SECURITY.md)
- Licença: [MIT](LICENSE) © SrSatriano 2026

---

<p align="center">Desenvolvido com foco em clareza e engenharia de produção · <a href="https://github.com/SrSatriano/mempool-arbitrage-mev-bot">Ver no GitHub</a></p>
