<div align="center">

# openfiat-devtools

**Test networks, protocol conformance vectors, benchmarks, and fuzzing for OpenFiat.**

[![CI](https://github.com/OpenFiat-org/openfiat-devtools/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenFiat-org/openfiat-devtools/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Discussions](https://img.shields.io/github/discussions/OpenFiat-org/openfiat-devtools)](https://github.com/orgs/OpenFiat-org/discussions)

[Website](https://openfiat.org) · [Docs](https://docs.openfiat.org) · [Specs](https://github.com/OpenFiat-org/openfiat-specs) · [Contributing](CONTRIBUTING.md)

</div>

---

## About

`openfiat-devtools` is part of the [OpenFiat](https://github.com/OpenFiat-org)
ecosystem — an open, decentralized peer-to-peer protocol for exchanging
stablecoins for local fiat currency. Solana secures asset settlement through
audited smart contracts; OpenFiat coordinates the peer-to-peer marketplace
layer (discovery, advertisements, reputation, governance, notifications, and
more) without centralized infrastructure.

This repository (Development) — test networks, protocol conformance vectors, benchmarks, and fuzzing for openfiat.

For the full protocol motivation and design, see the
[whitepaper](https://github.com/OpenFiat-org/openfiat-specs) and the
[protocol specifications](https://github.com/OpenFiat-org/openfiat-specs/tree/main/Whitepaper/Specifications).

## Repository layout

```
.
├── testnet/       # public test network: compose, genesis, node config, monitoring
├── devnet/        # single-command local dev network
├── conformance/   # Rust vector-loading harness + vectors/<area>/*.json
├── benchmarks/    # Criterion benchmarks (placeholder targets)
├── fuzzing/       # cargo-fuzz targets (placeholder targets)
├── docs/
└── examples/
```


## Quick start

```bash
# Local devnet
(cd devnet && docker compose up)

# Conformance vectors
(cd conformance && cargo test)

# Benchmarks
(cd benchmarks && cargo bench)
```


## Development

Docker-based network configs (`testnet/`, `devnet/`) require Docker with the
Compose plugin. Rust components (`conformance/`, `benchmarks/`, `fuzzing/`)
require a stable Rust toolchain; `fuzzing/fuzz/` additionally requires
nightly + `cargo-fuzz`.


## Testing

```bash
(cd conformance && cargo test)
(cd benchmarks && cargo check --benches)
docker compose -f testnet/docker-compose.yml config -q
docker compose -f devnet/docker-compose.yml config -q
```


## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) and
our [Code of Conduct](CODE_OF_CONDUCT.md) before opening a pull request.
Security issues should be reported per [SECURITY.md](SECURITY.md), not as
public issues.

See [ROADMAP.md](ROADMAP.md) for current priorities and
[CHANGELOG.md](CHANGELOG.md) for release history.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
