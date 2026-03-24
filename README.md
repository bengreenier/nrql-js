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

1. Create a repository (e.g. `bengreenier/nrql-js`).
2. Push this project to the `main` branch.
3. Optional: add branch protection and required status checks for the **CI** workflow (`.github/workflows/ci.yml`).

## npm (scoped package)

1. On [npmjs.com](https://www.npmjs.com/), create an organization or user scope that matches the package name (e.g. **`@nrql-js`**).
2. Create an **automation** or **granular** access token with permission to publish that scope.
3. In the GitHub repo: **Settings → Secrets and variables → Actions**, add **`NPM_TOKEN`** with that token.
4. Bump the `"version"` field in `package.json` to match the tag (e.g. `0.1.0` for tag `v0.1.0`), commit, then tag and push:

   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

The **Release** workflow (`.github/workflows/release.yml`) builds native addons on Linux, macOS, and Windows, merges the `.node` files, runs `napi create-npm-dirs` / `napi artifacts` / `napi prepublish`, and publishes to npm with `npm publish --ignore-scripts` so the package tarball matches the assembled artifacts.

To publish **manually** (not recommended if you use CI): build on each target platform (or copy `nrql-js.*.node` artifacts into the repo root), then:

```bash
pnpm run build
pnpm exec napi create-npm-dirs
pnpm exec napi artifacts --output-dir . --dist npm
pnpm exec napi prepublish -t npm
npm publish --access public
```

`package.json` includes `prepublishOnly` (`napi prepublish -t npm`); `npm publish` runs that automatically unless you pass `--ignore-scripts` (as the release workflow does after running those steps explicitly).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

`nom_nrql` is also dual-licensed under MIT OR Apache-2.0.
