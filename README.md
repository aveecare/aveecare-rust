# AveeCare Rust Library

[![crates.io](https://img.shields.io/crates/v/aveecare.svg)](https://crates.io/crates/aveecare)
[![docs.rs](https://img.shields.io/docsrs/aveecare)](https://docs.rs/aveecare)
[![CI](https://github.com/aveecare/aveecare-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/aveecare/aveecare-rust/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/aveecare/aveecare-rust/blob/main/LICENSE)

The official Rust library for the [AveeCare API](https://www.aveecare.com/docs/api).
Read and write your home care agency's patients, visits, invoices and more from your own
servers, with typed models, automatic pagination, retries and idempotency built in.

- Async on Tokio and reqwest (rustls), with an optional `blocking` client
- Typed models and builders, a typed `Error` enum
- Automatic retries with backoff and automatic idempotency keys

## Install

```sh
cargo add aveecare
cargo add tokio --features full
```

For the synchronous client: `cargo add aveecare --features blocking`.

## Quickstart

Create an API key in AveeCare, then:

```rust,no_run
use aveecare::{AveeCare, PatientLifecycleStage, PatientListParams};

#[tokio::main]
async fn main() -> Result<(), aveecare::Error> {
    let avee = AveeCare::new(std::env::var("AVEECARE_API_KEY").expect("set AVEECARE_API_KEY"));

    let page = avee
        .patients()
        .list(PatientListParams::new().lifecycle_stage(PatientLifecycleStage::Client).limit(25))
        .await?;
    for patient in page.data {
        println!("{} {} {}", patient.id, patient.first_name.unwrap_or_default(), patient.last_name.unwrap_or_default());
    }
    Ok(())
}
```

## Authentication

Every request is authenticated with your secret API key, sent as
`Authorization: Bearer ak_<keyId>_<secret>`. Pass it to `AveeCare::new`, or
set `AVEECARE_API_KEY` and call `AveeCare::from_env()`. Keys are read-only or
read and write; see
[Authentication](https://www.aveecare.com/docs/api/authentication).

Keep keys on your server. Never put an API key in browser or mobile code.

## Usage

```rust,no_run
use aveecare::{AveeCare, PatientCreate, PatientUpdate, VisitListParams};

async fn example(avee: &AveeCare) -> Result<(), aveecare::Error> {
    // Retrieve, create, update and delete
    let patient = avee.patients().retrieve("pat_8f2k1m9x").await?;
    let created = avee.patients().create(PatientCreate::new("Margaret", "Alvarez")).await?;
    let updated = avee
        .patients()
        .update(&created.id, PatientUpdate::new().scheduling_notes("Prefers morning visits").clear_email())
        .await?;
    avee.patients().delete(&created.id).await?;

    // Visits are listed by start-time window (at most 93 days)
    let visits = avee
        .visits()
        .list(VisitListParams::new("2026-10-01T00:00:00Z", "2026-10-08T00:00:00Z"))
        .await?;

    // The API key making the request
    let me = avee.me().retrieve().await?;
    Ok(())
}
```

Fields are snake_case (`first_name`, `created_at`) and map to the API's
camelCase JSON. Request types have a `new(...)` taking the required fields and
one setter per optional field; on updates, `clear_<field>()` sends `null`.
String enums have an `Unknown` variant so values added to the API later never
break decoding.

## Pagination

`list` returns one `Page`. Each list method has a `list_auto_paging` twin
returning a `Stream` of every item across every page:

```rust,no_run
use aveecare::futures_util::{pin_mut, StreamExt};
use aveecare::{AveeCare, PatientListParams};

async fn print_all(avee: &AveeCare) -> Result<(), aveecare::Error> {
    let patients = avee.patients().list_auto_paging(PatientListParams::new().limit(100));
    pin_mut!(patients);
    while let Some(patient) = patients.next().await {
        println!("{}", patient?.id);
    }
    Ok(())
}
```

To page by hand, pass `page.next_cursor` back with `.cursor(...)` while
`page.has_next_page()` is true.

## Errors

```rust,no_run
use aveecare::{AveeCare, Error};

async fn show(avee: &AveeCare) {
    match avee.patients().retrieve("pat_missing").await {
        Ok(patient) => println!("{}", patient.id),
        Err(Error::NotFound(details)) => println!("no such patient ({:?})", details.request_id),
        Err(err) => println!("{:?} {:?} {err}", err.status(), err.code()),
    }
}
```

| Status | Variant | Example codes |
|---|---|---|
| 400 | `Error::InvalidRequest` | `invalid_request`, `unknown_field` |
| 401 | `Error::Authentication` | `unauthorized` |
| 403 | `Error::Permission` | `forbidden`, `insufficient_scope` |
| 404 | `Error::NotFound` | `not_found` |
| 409 | `Error::Conflict` | `idempotency_conflict` |
| 429 | `Error::RateLimit` | `rate_limited` |
| other | `Error::Api` | `internal_error` |

Network failures are `Error::Connection`, and timeouts `Error::Timeout`.

## Retries, timeouts and idempotency

Rate-limited (429), server error (5xx) and network failures are retried twice
by default with jittered exponential backoff. When the API sends
`Retry-After`, the client waits that long.

Every `create` sends an `Idempotency-Key` (a random UUID), and the same key is
reused across retries, so a retried create never makes a duplicate. Pass your
own key to make a create safe to repeat across processes:

```rust,no_run
use std::time::Duration;
use aveecare::{AveeCare, PatientCreate, RequestOptions};

async fn import_row() -> Result<(), aveecare::Error> {
    let avee = AveeCare::builder()
        .api_key(std::env::var("AVEECARE_API_KEY").expect("set AVEECARE_API_KEY"))
        .max_retries(4)
        .timeout(Duration::from_secs(20))
        .build()?;

    avee.patients()
        .create_with_options(
            PatientCreate::new("Margaret", "Alvarez"),
            RequestOptions::new().idempotency_key("import-row-1042"),
        )
        .await?;
    Ok(())
}
```

## Blocking client

```rust,ignore
use aveecare::blocking::AveeCare;
use aveecare::PatientListParams;

fn main() -> Result<(), aveecare::Error> {
    let avee = AveeCare::from_env()?;
    for patient in avee.patients().list_auto_paging(PatientListParams::new()) {
        println!("{}", patient?.id);
    }
    Ok(())
}
```

Do not call the blocking client from inside an async runtime. This block needs
the `blocking` feature; see [examples/blocking.rs](examples/blocking.rs).

## Configuration

`AveeCare::builder()` accepts `api_key` (default `AVEECARE_API_KEY`),
`base_url` (default `AVEECARE_BASE_URL`, then `https://api.aveecare.com`),
`timeout` (60 seconds per attempt), `max_retries` (2), `http_client` (your own
`reqwest::Client`) and `default_header`. Every request sends
`User-Agent: aveecare-rust/<version>`.

## Documentation

The full API reference, with every resource and field, is at
[www.aveecare.com/docs/api](https://www.aveecare.com/docs/api), and the crate
docs are on [docs.rs](https://docs.rs/aveecare).
Building an integration to offer other agencies? See the
[AveeCare Partner Center](https://www.aveecare.com/partners).

## Other official SDKs

- [aveecare-node](https://github.com/aveecare/aveecare-node): Node.js and TypeScript
- [aveecare-python](https://github.com/aveecare/aveecare-python): Python
- [aveecare-rust](https://github.com/aveecare/aveecare-rust): Rust
- [aveecare-go](https://github.com/aveecare/aveecare-go): Go
- [aveecare-ruby](https://github.com/aveecare/aveecare-ruby): Ruby
- [aveecare-php](https://github.com/aveecare/aveecare-php): PHP
- [aveecare-java](https://github.com/aveecare/aveecare-java): Java and Kotlin
- [aveecare-dotnet](https://github.com/aveecare/aveecare-dotnet): .NET
- [aveecare-cli](https://github.com/aveecare/aveecare-cli): command line
- [aveecare-openapi](https://github.com/aveecare/aveecare-openapi): OpenAPI specification
- [aveecare-mcp](https://github.com/aveecare/aveecare-mcp): connect AI assistants
- [aveecare-examples](https://github.com/aveecare/aveecare-examples): sample apps

## License

MIT. See [LICENSE](https://github.com/aveecare/aveecare-rust/blob/main/LICENSE).

---

Built by [AveeCare](https://www.aveecare.com) — home care software.
