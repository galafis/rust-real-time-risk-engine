# rust-real-time-risk-engine

**A real-time risk engine in Rust for calculating Value at Risk (VaR) from live market data streams.**

---

## 🇧🇷 Descrição em Português

`rust-real-time-risk-engine` é um motor de risco em tempo real que se conecta a um feed de dados de mercado via WebSockets, consome os preços dos ativos e calcula o Value at Risk (VaR) em tempo real. Este projeto demonstra a capacidade do Rust de lidar com aplicações de baixa latência e processamento de streams, essenciais no setor financeiro.

Este é o quinto e último de uma série de cinco repositórios focados em trading, mercado financeiro e IA, mostrando como o Rust pode ser usado para construir sistemas de monitoramento de risco robustos e performáticos.

### Funcionalidades

- **Conectividade em Tempo Real:** Utiliza `tokio-tungstenite` para se conectar a feeds de dados de mercado via WebSockets.
- **Processamento de Streams:** Processa de forma assíncrona os dados de mercado recebidos.
- **Cálculo de Risco:** Implementa um cálculo simplificado de Value at Risk (VaR).
- **Arquitetura Assíncrona:** Construído sobre o runtime `tokio` para máxima performance e concorrência.

---

## 🇺🇸 English Description

`rust-real-time-risk-engine` is a real-time risk engine that connects to a market data feed via WebSockets, consumes asset prices, and calculates Value at Risk (VaR) in real time. This project demonstrates Rust's ability to handle low-latency, stream-processing applications, which are essential in the financial industry.

This is the fifth and final in a series of five repositories focused on trading, the financial market, and AI, showcasing how Rust can be used to build robust and high-performance risk monitoring systems.

### Features

- **Real-Time Connectivity:** Uses `tokio-tungstenite` to connect to market data feeds via WebSockets.
- **Stream Processing:** Asynchronously processes incoming market data.
- **Risk Calculation:** Implements a simplified Value at Risk (VaR) calculation.
- **Asynchronous Architecture:** Built on the `tokio` runtime for maximum performance and concurrency.

---

## 🚀 Quick Start

### Pré-requisitos

- Rust (https://www.rust-lang.org/tools/install)
- Git
- Uma chave de API da Finnhub (https://finnhub.io/)

### Instalação

1. Clone o repositório:
```bash
git clone https://github.com/your-username/rust-real-time-risk-engine.git
cd rust-real-time-risk-engine
```

2.  **Importante:** Substitua o placeholder `YOUR_FINNHUB_API_KEY` no arquivo `examples/real_time_risk.rs` pela sua chave de API da Finnhub.

3. Compile e execute o exemplo:
```bash
cargo run --example real_time_risk
```

### Exemplo de Saída

O motor de risco irá se conectar ao feed da Finnhub, se inscrever no ticker da Apple (AAPL) e começar a calcular o VaR para cada novo preço recebido.

```
[INFO  rtre_utils] Logger initialized.
[INFO  rtre_market_data] Connected to market data feed.
[INFO  rtre_market_data] Subscribed to ticker: AAPL
[INFO  rtre_risk_calculator] Calculated VaR: 4.123
[INFO  rtre_risk_calculator] Calculated VaR: 4.125
...
```

---

## 🏛️ Arquitetura

O sistema é projetado para ser modular e escalável, com uma clara separação de preocupações.

![Arquitetura do Motor de Risco](https://i.imgur.com/Lz3b4k2.png)

### Crates

- `rtre-core`: Orquestra o fluxo de dados e os cálculos de risco.
- `rtre-market_data`: Gerencia a conexão e o consumo de dados do WebSocket.
- `rtre-risk_calculator`: Contém a lógica para os cálculos de risco (VaR).
- `rtre-utils`: Funções utilitárias.

---

## 🛣️ Roadmap

- [ ] Implementar cálculos de risco mais sofisticados (ex: Expected Shortfall - ES, Stress Testing).
- [ ] Adicionar suporte para múltiplos feeds de dados e múltiplos tickers simultaneamente.
- [ ] Criar um dashboard para visualização dos cálculos de risco em tempo real.
- [ ] Permitir a configuração de parâmetros de risco (ex: nível de confiança, janela de tempo) via arquivo de configuração ou API.
- [ ] Integração com os outros repositórios da série para uma plataforma de trading completa.

---

## 🤝 Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir uma issue ou enviar um pull request.

---

## 📜 Licença

Este projeto está licenciado sob a licença MIT.

---

## 👨‍💻 Autor

**Gabriel Demetrios Lafis**

*   Cientista de Dados | Analista de Dados | BI/BA
*   Formado em Análise e Desenvolvimento de Sistemas, Gestão da Tecnologia da Informação e Segurança Cibernética.

