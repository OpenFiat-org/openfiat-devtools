# testnet

A public OpenFiat test network: one bootstrap `openfiat-node` plus a
Prometheus/Grafana pair for monitoring, orchestrated with Docker Compose
(see `../../openfiat-infra` for the node's own Dockerfile).

```bash
docker compose up
```

`openfiat-node` has no CLI flags or config file — every setting is an
environment variable, set in `node.env` here (see `openfiat-core`'s
`crates/cli`). There is no genesis-file concept to configure: protocol
parameters (staking minimums, unbonding period, quorum, etc.) are on-chain
Anchor account state (`openfiat-staking`/`openfiat-governance`, initialized
once via a deploy script), not something a node reads from disk.

Other operators join by pointing their own node's `CLI_BOOTSTRAP_PEERS` at
this bootstrap node's multiaddr (a static IP/multiaddr, not a hostname —
this workspace doesn't enable libp2p's `dns` feature).

Grafana/Prometheus scrape the node's own `/metrics` — today that's just
`rpc_requests_total`/`rpc_errors_total`; there's no peer-count or
sync-state metric yet, only JSON-RPC (`getChainStatus`, `getLatestSnapshot`,
`getCheckpointHeight`).

```bash
./scripts/reset.sh   # docker compose down -v && up -d
```
