# Titan Audit

Production-ready polyglot (Rust + Go + Python) compliance & audit engine for enterprise AI. Immutable append-only ledger, policy engine, self-healing agents, evidence generator, and security/privacy/financial agent swarms. Built for SOC 2, HIPAA, GDPR, PCI-DSS, NIST.

**TL;DR**: Build once, deploy safely, get a verified artifact every time. No placeholders, no to-dos, no errors.

---

## Quick Start

```bash
bash make_titan_audit.sh titan_audit
cd titan_audit
bash run.sh
```

---

## Architecture

- **Rust Core**: Immutable ledger, hash chaining, Merkle tree, cryptographic proofs.
- **Go Runtime**: Policy engine, drift detection, self-healing agents, evidence generator.
- **Python Swarm**: Security agent, privacy agent, financial agent with orchestration.
- **Audit Layer**: Append-only log, Merkle roots, integrity verification.
- **Deployment**: Build, package, checksum verify, safe extract, smoke test.

---

## Tech Details

- **Rust**: `titan_audit_core` crate with `extern "C"` ABI for FFI.
- **Go**: `cgo` bindings to Rust core for ledger operations.
- **Python**: `pyo3` + `cffi` for Rust core bindings.
- **Immutable**: Append-only events, hash chaining, tamper detection.
- **Audit**: SHA-256, Merkle tree, proof of inclusion.

---

## Build

```bash
bash build.sh
```

## Deploy

```bash
bash deploy.sh
```

## Smoke Test

```bash
python3 output/titan_audit/deploy/current/python/main.py --healthcheck
```

---

## License

MIT

---

## Security

See SECURITY.md.

---

## Contributing

See CONTRIBUTING.md.
