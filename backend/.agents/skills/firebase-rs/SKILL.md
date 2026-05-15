---
name: firebase-rs
description: Build or modify Rust projects that use the firebase-rs crate for Firebase Realtime Database REST access, including unauthenticated/authenticated clients, nested paths, typed reads, writes, updates, deletes, query parameters, and realtime Server-Sent Events streams. Use when the user mentions firebase-rs, firebase_rs, Rust Firebase Realtime Database, Firebase::new/auth/at/get/set/update/delete, or wants Rust code using Firebase Realtime Database via this crate.
---

# Firebase RS

## Workflow

Use this skill for `firebase-rs`, the Rust crate exposed as `firebase_rs`, when implementing Firebase Realtime Database access through its REST API wrapper.

1. Inspect the project first: confirm async runtime, error handling style, config/env conventions, and whether `serde` models already exist.
2. Add dependencies conservatively. Typical app dependencies are `firebase-rs`, `serde` with `derive`, and an async runtime such as `tokio`.
3. Keep Firebase database URLs and auth tokens in config or environment variables. Do not hardcode secrets in examples or committed code.
4. Build paths with chained `.at("segment")` calls instead of manual URL string concatenation.
5. Prefer typed `get::<T>()`, `set(&value)`, and `update(&value)` over raw strings unless the caller explicitly wants raw JSON/text.
6. Match existing project error style. Avoid `.unwrap()` in production code unless nearby code uses panics for setup failures.
7. Run the Rust formatter and the relevant tests/checks after edits.

## Project Setup

Check the latest crate version before pinning if the user asks for current dependencies. The crate name in `Cargo.toml` is `firebase-rs`; the Rust module path is `firebase_rs`.

Typical dependency shape:

```toml
[dependencies]
firebase-rs = "2"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Use `serde_json` only when the app needs dynamic JSON values or partial update maps.

## Implementation Notes

- `Firebase::new(url)` creates a client without auth.
- `Firebase::auth(url, token)` creates a client with an auth key/token.
- `.at("users").at(user_id)` returns a new scoped `Firebase` path.
- `.get::<T>().await` deserializes into `T`; collections commonly use `HashMap<String, T>`.
- `.get_as_string().await` returns the raw response when typed deserialization is not appropriate.
- `.set(&value).await` pushes data under the current path with a Firebase-generated key.
- `.set_with_key(key, &value).await` writes data with a caller-provided child key and requires a mutable scoped client.
- `.update(&value).await` patches data at the current path.
- `.delete().await` deletes data at the current path.
- `.with_params().order_by(...).start_at(...).equal_to(...).finish()` builds query parameterized clients.
- `.with_realtime_events()` creates Server-Sent Events support for realtime database streams.

For exact snippets and method patterns, read `references/api-patterns.md`.

## Guardrails

This crate targets Firebase Realtime Database REST usage. Do not present it as a full Firebase Admin SDK, Firestore client, Authentication management SDK, Storage SDK, or FCM library.

Firebase Realtime Database auth varies by project rules and token type. If the user has not specified auth, implement the config boundary clearly and leave token acquisition to the surrounding application unless that is explicitly in scope.
