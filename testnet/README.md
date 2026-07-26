# testnet

Public OpenFiat test network: bootstrap node configuration, genesis
parameters, and monitoring, orchestrated with Docker Compose.

```bash
docker compose up
```

This currently launches placeholder service definitions wired to
`openfiat-core`'s `openfiat-node` binary image (see `../../openfiat-infra`
for the Dockerfile) plus a Prometheus/Grafana pair for monitoring. Genesis
parameters live in `genesis.json`.
