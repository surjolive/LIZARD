# Lizard Package Ecosystem Specification

**Status:** PHASE 1 specification only
**Language:** Lizard
**Source extension:** `.lz`
**CLI:** `lizard`
**Creator/Maintainer:** Surjo Deb Nath
**GitHub owner:** SURJO99exe
**Target repository:** https://github.com/SURJO99exe/lizard
**Registry:** `packages.lizard.dev` (planned)
**License:** The repository's selected license applies; package licenses remain
package metadata and must be respected.

This document defines the package ecosystem contract. It does not claim that
registry, publishing, authentication, or dependency resolution are already
implemented in the current interpreter.

## 1. Goals and Non-Goals

The ecosystem will provide an original package manager, registry, storage
layer, metadata database, resolver, build system, publishing workflow, search,
documentation, versioning, and security verification for Lizard packages.

It will:

- make local packages reproducible;
- make published packages discoverable and verifiable;
- keep source packages portable across Windows, Linux, and macOS;
- separate registry metadata from package archives;
- support public, private, and organization packages later;
- preserve lockfile reproducibility after a release is yanked.

It will not copy source code or proprietary implementation details from PyPI,
npm, crates.io, RubyGems, Go modules, or other registries.

## 2. Official Package Layout

Every publishable package uses this layout:

```text
my_package/
├── lizard.toml
├── lizard.lock                 # generated for applications when needed
├── src/
│   └── main.lz                 # package entry module
├── tests/
│   └── test_main.lz
├── examples/
├── docs/
├── README.md
├── LICENSE
└── CHANGELOG.md
```

Required for publishing: `lizard.toml`, `src/`, `README.md`, and `LICENSE`.
Tests are strongly recommended and may be required by a future registry policy.
Archives must exclude build output, credentials, editor caches, and files
outside the package root.

## 3. `lizard.toml` Manifest

The manifest is TOML and has one authoritative schema:

```toml
[package]
name = "my_math"
version = "1.0.0"
description = "Math library for Lizard"
authors = ["Surjo Deb Nath"]
license = "MIT"
repository = "https://github.com/SURJO99exe/my_math"
homepage = "https://packages.lizard.dev/packages/my_math"
keywords = ["math", "numbers"]
categories = ["mathematics"]

[package.targets]
source = "src/main.lz"

[dependencies]
array = "^1.0.0"
http = "~2.0.0"

[dev-dependencies]
testing = "^1.0.0"

[build]
entry = "src/main.lz"
required_lizard = ">=0.1.0"
platforms = ["windows-x86_64", "linux-x86_64", "macos-arm64"]
```

Reserved top-level tables are `[package]`, `[dependencies]`,
`[dev-dependencies]`, `[build]`, and future `[workspace]` and `[features]`.
Unknown fields produce a manifest warning initially and an error after the
schema becomes stable.

## 4. Package Names and Namespaces

Public package names are lowercase ASCII identifiers of 1-64 characters. They
may contain letters, digits, single hyphens, or underscores, must start with a
letter, and must not end with a separator. Names are normalized for lookup but
preserve display metadata.

Examples: `http`, `json_tools`, `web-server`.

Disallowed names include path traversal, control characters, repeated
separators, names that only differ by Unicode normalization, reserved CLI
commands (`install`, `publish`, `update`, and similar), and names intended to
impersonate first-party packages. Namespaces use `@owner/name`, for example
`@lizard/array` or `@surjo/tools`. Ownership and name uniqueness are registry
responsibilities.

## 5. Versions and Dependency Constraints

Versions use `MAJOR.MINOR.PATCH` semantic versioning, with optional prerelease
and build metadata. Release versions are immutable.

Supported constraints:

```text
1.2.3       exact version
^1.2.0      >=1.2.0 and <2.0.0
~1.2.0      >=1.2.0 and <1.3.0
>=1.0.0
<2.0.0
```

The resolver selects the highest non-yanked version satisfying every direct and
transitive constraint, unless the lockfile already pins a valid version. A
prerelease is never selected for a stable constraint unless explicitly named.
Conflicts produce `DependencyConflict` with the complete constraint chain.
Circular dependencies produce `CircularDependency` with the cycle path.

Optional and platform dependencies are planned through manifest features:

```toml
[features]
postgres = ["postgres-driver"]

[target.linux.dependencies]
linux-native = "^1.0.0"
```

## 6. Lockfile and Reproducibility

`lizard.lock` records the resolved graph:

```toml
[lock]
format = 1
lizard = ">=0.1.0"

[[package]]
name = "array"
version = "1.2.0"
source = "registry+https://packages.lizard.dev"
checksum = "sha256:..."
dependencies = []

[[package]]
name = "http"
version = "2.0.1"
source = "registry+https://packages.lizard.dev"
checksum = "sha256:..."
dependencies = ["network 1.4.0"]
```

Install uses the lockfile when constraints remain compatible. Update is the
explicit operation that changes pins. Checksum, source, exact version, and
transitive dependencies are required for a reproducible install.

## 7. Archive Format

The official archive extension is `.lzpkg`. The canonical archive name is
`name-version.lzpkg`. It contains normalized UTF-8 paths, no absolute paths,
no `..` components, and these entries:

```text
manifest/lizard.toml
source/**
tests/**
examples/**
docs/**
README.md
LICENSE
CHANGELOG.md
checksums/sha256.txt
```

Archives are reproducibly created with sorted paths, normalized metadata, and
stable compression settings. The registry stores the archive outside database
rows and exposes its checksum and size as metadata.

## 8. CLI Contract

The future package commands are:

```text
lizard init [name]
lizard create lib <name>
lizard install <package>[@version]
lizard uninstall <package>
lizard update
lizard upgrade
lizard search <query>
lizard info <package>[@version]
lizard list
lizard outdated
lizard build
lizard test
lizard package
lizard publish
lizard yank <package>@<version>
lizard login
lizard logout
lizard whoami
lizard owner <package>
lizard docs
lizard version
lizard cache clean
```

`lizard install` reads the manifest, contacts the configured registry, resolves
dependencies, downloads archives, verifies checksums, installs into the local
cache, updates the manifest only when explicitly requested, and writes the
lockfile. It reports each step and gives actionable errors.

`lizard install --offline` uses only verified cache entries and the lockfile;
missing artifacts produce `OfflineDependencyError`. Registry configuration is
planned as:

```text
lizard config set registry https://packages.lizard.dev
lizard config set registry http://localhost:8000
```

## 9. Local Cache, Mirrors, and Platforms

The default cache is `~/.lizard/cache/`, with platform-appropriate expansion.
A cached package is reused only after checksum verification. Trusted mirrors
may be configured per user or organization, but each source must be explicit
in the lockfile. Registry metadata supports Windows, Linux, macOS, x86_64,
and arm64 requirements.

## 10. Resolver and Build System

The resolver performs manifest validation, graph construction, constraint
intersection, cycle detection, platform filtering, optional dependency
selection, and deterministic ordering. It never executes package code during
resolution.

`lizard build` supports pure Lizard packages first. Native Rust, C, and C++
extensions are a later capability with an explicit FFI/ABI version, platform
matrix, reproducible build inputs, and permission review. Package builds must
not silently download arbitrary executables or run shell scripts.

## 11. Registry Architecture

```text
Lizard CLI
    -> Registry REST API
        -> PostgreSQL metadata database
        -> Object storage for .lzpkg archives
        -> Search index
        -> Documentation renderer
```

The storage abstraction supports local filesystem for development and
S3-compatible/cloud object storage for deployment. Large archives are never
stored directly in PostgreSQL rows.

### REST API v1

```text
POST /api/auth/register
POST /api/auth/login
POST /api/auth/tokens
POST /api/packages/publish
GET  /api/packages/search?q=array
GET  /api/packages/{name}
GET  /api/packages/{name}/versions
GET  /api/packages/{name}/{version}
GET  /api/packages/{name}/{version}/download
POST /api/packages/{name}/owners
DELETE /api/packages/{name}/owners/{user}
POST /api/packages/{name}/{version}/yank
```

The API returns structured errors, request IDs, pagination, rate-limit headers,
and no secret tokens in responses except at explicit token creation.

## 12. Registry Database

The PostgreSQL schema contains:

- `users`: username, email, password hash, profile, timestamps;
- `packages`: canonical name, namespace, description, repository, visibility;
- `package_versions`: package, semantic version, archive key, checksum, size,
  yanked/deprecated flags, publisher, timestamps;
- `dependencies`: package version, dependency name, constraint, optional and
  platform selectors;
- `owners` and `maintainers`: package/user roles and authorization state;
- `releases`: immutable publication events;
- `documentation`: rendered README/API docs by version;
- `downloads`: aggregated counters by package/version/time bucket;
- `api_tokens`: hashed, scoped, revocable token records;
- `audit_logs`: publish, yank, ownership, maintainer, and security events.

Indexes cover canonical package lookup, full-text search, version ordering,
dependency lookup, download aggregation, and audit time ranges.

## 13. Accounts, Tokens, and Ownership

Passwords are hashed with an established password-hashing library and are
never stored or logged in plaintext. Sessions use secure expiration and CSRF
protection where applicable. `lizard token create` displays a token once;
registry storage keeps only a verifier/hash. Tokens have scopes, expiration,
and revocation.

Every package has an owner and may have approved maintainers and contributors.
Ownership changes require authorization and create audit events. Organizations
such as `@lizard` and `@surjo` are planned with member roles and package
permissions.

## 14. Publishing and Verification

The publishing workflow is:

```text
lizard login
lizard test
lizard build
lizard package
lizard publish
```

Before accepting a release the registry validates package name, version,
manifest, structure, required files, dependency constraints, archive size,
license, tests, and duplicate versions. Existing versions are rejected and
published archives are immutable.

Every archive has a SHA-256 checksum. Optional release signing uses an
established cryptographic library and publisher identity; no custom
cryptography is permitted. Installation rejects checksum mismatches. Security
scanning may inspect suspicious files, dangerous metadata, known vulnerable
dependencies, and anomalies, but never claims perfect malware detection.

Yanking prevents new resolution while preserving existing lockfile installs
where possible. Deprecation metadata may name a replacement package. Neither
operation silently deletes an immutable archive.

## 15. Search, Website, and Documentation

Search indexes name, description, keywords, author, category, versions, and
quality signals. Ranking must be explainable and must not deceptively
manipulate results. Categories include Web, Networking, AI, Machine Learning,
Data Science, Mathematics, Database, GUI, Game, CLI, Automation, Security,
and Utilities.

The planned website at `packages.lizard.dev` includes `/search`, `/packages`,
`/package/{name}`, `/package/{name}/{version}`, `/login`, `/register`,
`/dashboard`, `/publish`, and `/settings`. Package pages show metadata,
maintainers, license, dependencies, install command, documentation, changelog,
versions, downloads, repository, homepage, and issues.

README and documentation rendering uses a safe Markdown renderer with HTML and
script sanitization, URL policy, content-size limits, and no execution of
package code. `lizard docs` produces package API documentation from source
comments when that compiler feature is available.

## 16. Rate Limits, Privacy, and Audit

The registry rate-limits login, registration, search, metadata, download, and
publishing endpoints. Errors include retry guidance. Download statistics track
total, version, daily, weekly, and monthly counts while minimizing personal
information. Administrative audit logs are append-oriented and protected from
ordinary package-user modification.

## 17. Recommended Implementation Stack

The first implementation should reuse the existing Rust Lizard CLI and use a
small, tested registry service. A practical deployment is:

- CLI/resolver: Rust, sharing semantic-version and archive validation logic;
- registry API: Rust or Python with explicit OpenAPI documentation;
- database: PostgreSQL;
- storage: local filesystem in development, S3-compatible storage in
  production;
- frontend: React/Next.js or another maintainable web stack;
- local development: Docker Compose;
- CI: GitHub Actions.

Technology choices may change only with a documented tradeoff and migration
plan.

## 18. Monorepo Layout

```text
Lizard-Ecosystem/
├── cli/{commands,install,resolver,publish}/
├── registry/{api,auth,packages,database,storage,security}/
├── frontend/
├── package-format/
├── docs/
├── examples/
├── tests/
├── docker/
├── scripts/
├── README.md
├── LICENSE
└── CONTRIBUTING.md
```

## 19. End-to-End Acceptance Example

Phase 1 defines this workflow; it is not yet claimed as working in the
current repository:

```text
lizard create lib my_math
# add src/main.lz and tests/test_main.lz
lizard test
lizard build
lizard package
lizard login
lizard publish

lizard init my_app
lizard install my_math
# import my_math from Lizard source
```

The end-to-end test is complete only when it runs against a local registry,
verifies the archive, writes a lockfile, installs from cache, and reproduces
the same dependency graph offline.

## 20. Implementation Roadmap

1. **Manifest parser:** parse and validate `lizard.toml` with tests.
2. **Local package model:** canonical names, versions, package layout.
3. **CLI install/uninstall:** local cache and safe archive extraction.
4. **Version/resolver:** semantic constraints, conflicts, cycles, platforms.
5. **Lockfile:** deterministic graph serialization and reproducible installs.
6. **Archive format:** canonical `.lzpkg`, checksums, path safety.
7. **Local registry:** development API and filesystem storage.
8. **PostgreSQL registry:** schema, indexes, migrations, audit events.
9. **Publishing:** validation, immutable versions, yanking, deprecation.
10. **Authentication:** accounts, scoped tokens, ownership and maintainers.
11. **Search/docs website:** safe rendering, search, package pages.
12. **Security:** signatures, scanning boundaries, rate limits, private packages.
13. **Caching/offline/mirrors:** verified cache and trusted sources.
14. **Native extensions:** explicit FFI/ABI and reproducible platform builds.
15. **First-party packages:** `@lizard/array`, `@lizard/data`, `@lizard/math`,
    `@lizard/http`, `@lizard/plot`, `@lizard/database`, `@lizard/ml`, and
    `@lizard/vision` as separately tested packages.
16. **Production deployment:** Docker, CI/CD, backups, monitoring, and release
    operations.

Every phase must include specification updates, changed files, project tree,
working code or `NOT IMPLEMENTED`, tests, examples, run instructions,
limitations, TODOs, and the next phase. No placeholder is complete until its
acceptance tests pass.
