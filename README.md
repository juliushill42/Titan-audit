# OpenClaw Monolith

A production-ready, single-repo AI ops platform that turns OpenClaw from a "cool demo" into a serious, deployable system with audit trails, safe extraction, checksum verification, and smoke-tested deployment.

**TL;DR**: Build once, deploy safely, and get a verified artifact every time. No placeholders, no to-dos, no errors.

---

## What This Is

- A **monolithic codebase** that builds a single release artifact.
- An **audit ledger** that records every file, hash, and change.
- **Safe tar extraction** that rejects path traversal and symlinks.
- **Checksum verification** before deployment.
- A **smoke test** that ensures the deploy target is healthy.
- **GitHub-ready** with CI, secret scanning, and branch protection.

---

## Why This Matters

OpenClaw is a powerful personal agent, but it feels like a toy because:
- Credentials and shell access can be exposed by bad prompts.
- Skills/plugins are unsafe or malicious.
- There's no approval model or audit trail.
- It doesn't scale beyond the builder's workflow.

This monolith fixes those gaps:
- **Default-deny** access with scoped permissions.
- **Safe extraction** and verification of release artifacts.
- **Audit manifests** and deployment logs for every release.
- **Smoke-tested** deployment and rollback paths.

---

## Quick Start

1. Create the repo:
   ```bash
   bash make_repo.sh openclaw-monolith
   cd openclaw-monolith
   ```
2. Build and deploy:
   ```bash
   bash run.sh
   ```
   You'll see a `build.log`, `deploy.log`, and `release.json` in `output/openclaw-monolith/`.

3. Test the healthcheck:
   ```bash
   python3 output/openclaw-monolith/deploy/current/app/main.py --healthcheck
   ```

---

## For Non-Technical Readers

- **What it does**: It builds and deploys a single, verified system that automates tasks safely.
- **What you get**: A clean release with logs, a manifest, and a checksum you can trust.
- **How to use it**: Run `bash run.sh`. If it finishes, the system is deployed and healthy.

---

## For Technical Readers

### Architecture

- `app/` — Python runtime: main, tests, record, package, checksum, verify, safe_extract.
- `build.sh` — Audit, lint, test, package, and checksum.
- `deploy.sh` — Verify, extract safely, and smoke test.
- `run.sh` — Build and deploy in one command.
- `output/` — Build artifacts, logs, and deployment target.
- `.github/workflows/ci.yml` — CI that runs build, test, and deploy smoke test on push/PR.

### Safety

- **Path traversal protection**: `safe_extract.py` rejects paths outside the target.
- **No symlinks**: `issym` and `islnk` checks prevent escape.
- **Checksum verification**: `verify.py` ensures artifact integrity.
- **Audit manifest**: `record.py` captures file inventory with hashes.

### Modularity

While monolithic, the design keeps modules loosely coupled:
- Tests, packaging, and verification are separate functions.
- Each stage is a single Python module.
- CI gates the pipeline with build + smoke test.

### CI

GitHub Actions runs:
- `bash ./build.sh`
- `bash ./deploy.sh`

Failing any stage blocks the PR or push.

---

## Repository Hygiene

- README, LICENSE, SECURITY.md, CONTRIBUTING.md, .gitignore, Codeowners.
- Dependabot updates for GitHub Actions.
- CI workflow with path filters and pinned actions.

---

## Security Policy

- Do not commit secrets.
- Use GitHub Secrets or a vault for credentials.
- Report security issues privately via SECURITY.md.

---

## Contributing

- No secrets in commits.
- All changes must pass build and deploy.
- Keep scripts deterministic and auditable.
- Prefer small, reviewable commits.

---

## License

MIT License. See LICENSE for details.

---

## Credits

- Author: Julius Hill
- Focus: Secure AI ops, safe deployment, auditability, production-grade monolith.
