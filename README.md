# @nrql-js/nrql

Node.js bindings for **[nom_nrql](https://github.com/bengreenier/nom_nrql)** — a streaming [NRQL](https://docs.newrelic.com/docs/nrql/nrql-syntax-clauses-functions/) (New Relic Query Language) parser built with [nom](https://github.com/rust-bakery/nom). Native parsing is implemented in Rust via [NAPI-RS](https://napi.rs/).

## Install

```bash
pnpm add @nrql-js/nrql
# or: npm install @nrql-js/nrql
```

Prebuilt binaries are published as optional platform packages (same pattern as other NAPI-RS projects). If install fails to load a native addon, reinstall dependencies or build from source (below).

## Usage

```js
import { parseNrql } from '@nrql-js/nrql'

const ast = parseNrql('FROM Transaction SELECT count(*)')
console.log(ast.from.eventTypes) // ['Transaction']
```

Parse errors throw a normal `Error` whose message includes the parser message and byte offset when available.

Types are generated from the Rust bindings (`parseNrql` returns a `Query` and nested AST types). Run `pnpm run build` after cloning to generate `index.d.ts` locally.

## Development

Requirements: **Rust** (stable), **Node.js 18+**, **pnpm**.

```bash
git clone https://github.com/bengreenier/nrql-js.git
cd nrql-js
pnpm install
pnpm run build    # generates index.js, index.d.ts, and nrql-js.*.node
pnpm test
```

`index.js` and `index.d.ts` are build outputs and are gitignored; CI and publish always run the build first.

## GitHub

1. Repository: [bengreenier/nrql-js](https://github.com/bengreenier/nrql-js).
2. Optional: branch protection and required checks for **CI** (`.github/workflows/ci.yml`).
3. **Settings → Actions → General**: enable **Allow GitHub Actions to create and approve pull requests** (needed for [Release Please](https://github.com/googleapis/release-please-action)).

## Releases (Release Please + npm)

This repo uses **[Release Please](https://github.com/googleapis/release-please)** ([workflow](.github/workflows/release-please.yml)) with **[Conventional Commits](https://www.conventionalcommits.org/)** on `main`:

| Commit prefix | Typical SemVer bump |
|----------------|---------------------|
| `fix:` … | patch |
| `feat:` … | minor |
| `feat!:` / `fix!:` / … `!` | major (breaking) |

Release Please opens a **Release PR** that updates `CHANGELOG.md`, `package.json`, `.release-please-manifest.json`, and `Cargo.toml` (via `release-please-config.json`). When you **merge** that PR, it creates a **GitHub Release** and **git tag** `v*`.

### Secrets (Actions)

| Secret | Purpose |
|--------|---------|
| **`NPM_TOKEN`** | [npm](https://www.npmjs.com/) automation or granular token allowed to publish **`@nrql-js`**. Used by the Release Please workflow to run `npm publish`. |

### npm org

Create the **`@nrql-js`** scope on npm if needed, then add **`NPM_TOKEN`** as above.

### End-to-end flow

1. Land conventional commits on `main` (`feat:`, `fix:`, etc.).
2. Release Please updates the Release PR until you merge it.
3. When a release is created, the same **Release Please** workflow builds native addons (Linux / macOS / Windows) and publishes to npm.

If `Cargo.lock` drifts after a version bump in `Cargo.toml`, run `cargo build` and commit the lockfile on the Release PR branch (or a follow-up commit).

### Manual publish (advanced)

To publish without Release Please, first align `package.json`, `.release-please-manifest.json`, and `Cargo.toml`, then run:

Fully local publish (you must supply all platform `nrql-js.*.node` files yourself or accept a single-platform build):

```bash
pnpm run build
pnpm exec napi create-npm-dirs
pnpm exec napi artifacts --output-dir . --npm-dir npm
pnpm exec napi prepublish -t npm
npm publish --access public
```

The Release Please workflow uses `npm publish --ignore-scripts` after those `napi` steps because it runs `napi prepublish` explicitly first.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

`nom_nrql` is also dual-licensed under MIT OR Apache-2.0.
