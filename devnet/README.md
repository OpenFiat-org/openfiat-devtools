# devnet

Single-command local OpenFiat development network — one isolated
`openfiat-node` (no bootstrap peers configured, so it never dials out),
intended for local application development against a real node's JSON-RPC
surface rather than mocks.

```bash
docker compose up
```

The node has no CLI flags or config file of its own — every setting is an
environment variable (`crates/cli` in `openfiat-core`), set in
`docker-compose.yml` here. On first run it generates its own wallet.json
(reused as both its Solana identity and its libp2p keypair) into the
`openfiat-devnet-data` volume, so restarting `docker compose up` keeps the
same node identity and data.

```bash
# Health check
curl -s http://localhost:7080/health

# JSON-RPC
curl -s -X POST http://localhost:7080/rpc -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"getChainStatus","params":{}}'
```

`CLI_SOLANA_RPC_URLS` is left unset, so this node stays `GossipOnly` — it
never talks to Solana. Set it if you need `RpcConnected` behavior locally.
