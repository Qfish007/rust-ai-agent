# expense-tracker-api

A minimal CRUD web API in Rust (axum) with an **in-memory** store — no
database, no file persistence. Data is seeded when the process starts
and lives only for as long as it runs. This is the "existing internal
Rust web app" half of the MCP episode: a separate MCP server (built
next) will be a thin client over this API via HTTP, instead of talking
to a database directly.

## Stack

- Rust **2024** edition
- [`axum`](https://docs.rs/axum) `0.8` — routing + extractors
- `tokio` — async runtime
- `serde` / `serde_json` — (de)serialization
- `uuid` — expense IDs
- `chrono` — dates
- `tower-http` — request tracing

## Run it

```bash
cargo run
```

Starts on `http://localhost:3000`. `RUST_LOG=info` (or `debug`) turns on
request logging via `tracing`:

```bash
RUST_LOG=info cargo run
```

## Auth

Every route under `/expenses` requires this header:

```
x-api-key: dev-secret-key
```

It's a single hardcoded key on purpose — the point of the episode is
the CRUD + MCP wiring, not building a real auth system. `/health`
is intentionally left open so you can confirm the process is alive
without a key.

## Data model

```jsonc
{
  "id": "uuid",
  "description": "string",
  "amount": 42.50,
  "category": "string",
  "date": "2026-07-02"   // YYYY-MM-DD
}
```

## Endpoints

| Method | Path                 | Auth | Description                           |
|--------|----------------------|------|---------------------------------------|
| GET    | `/health`            | no   | Liveness check                        |
| GET    | `/expenses`          | yes  | List, optional `?category=` `?month=` |
| GET    | `/expenses/{id}`     | yes  | Get one                               |
| POST   | `/expenses`          | yes  | Create                                |
| PUT    | `/expenses/{id}`     | yes  | Partial update                        |
| DELETE | `/expenses/{id}`     | yes  | Delete                                |
| GET    | `/expenses/summary`  | yes  | Total + per-category breakdown, optional `?month=` |

## Try it with curl

```bash
KEY="dev-secret-key"

# health check, no auth needed
curl http://localhost:3000/health

# list everything
curl -H "x-api-key: $KEY" http://localhost:3000/expenses

# filter by category and/or month
curl -H "x-api-key: $KEY" "http://localhost:3000/expenses?category=Food"
curl -H "x-api-key: $KEY" "http://localhost:3000/expenses?month=2026-07"

# create
curl -X POST -H "x-api-key: $KEY" -H "Content-Type: application/json" \
  -d '{"description":"Coffee with a client","amount":9.5,"category":"Food","date":"2026-07-20"}' \
  http://localhost:3000/expenses

# get one (replace <id> with an id from the list/create response)
curl -H "x-api-key: $KEY" http://localhost:3000/expenses/<id>

# partial update — only sends the field(s) you want changed
curl -X PUT -H "x-api-key: $KEY" -H "Content-Type: application/json" \
  -d '{"amount":11.0}' \
  http://localhost:3000/expenses/<id>

# delete
curl -X DELETE -H "x-api-key: $KEY" http://localhost:3000/expenses/<id>

# summary
curl -H "x-api-key: $KEY" "http://localhost:3000/expenses/summary?month=2026-07"

# missing/wrong key -> 401
curl -i http://localhost:3000/expenses
```

## Project layout

```
src/
  main.rs      — router wiring, in-memory state, which routes need auth
  auth.rs      — the x-api-key middleware
  models.rs    — Expense + request/response types
  store.rs     — Arc<RwLock<HashMap<Uuid, Expense>>>, seeded on startup
  handlers.rs  — the actual CRUD + summary logic
  error.rs     — one AppError type mapped to HTTP status + JSON body
```

## Note on this sandbox

This project was written and logic-reviewed against a downgraded
axum 0.7 / edition 2021 copy in this container (its own Rust toolchain
is too old to understand edition 2024, and the installer for a newer
one isn't reachable from here). The only difference between that check
and the file you have is axum's path-param syntax (`:id` → `{id}`
going from 0.7 → 0.8) — everything else is unchanged. Run `cargo build`
on your Mac (Rust 1.85+) to get a real compile check.
