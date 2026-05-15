# firebase-rs API Patterns

Use these patterns when implementing Rust code with `firebase-rs` / `firebase_rs`.

## Imports and Models

```rust
use firebase_rs::Firebase;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
}
```

## Client Creation

```rust
let firebase = Firebase::new(&database_url)?;
```

```rust
let firebase = Firebase::auth(&database_url, &auth_token)?;
```

Prefer passing URL/token through the app's existing config type. For binaries, env vars such as `FIREBASE_DATABASE_URL` and `FIREBASE_AUTH_TOKEN` are reasonable.

## Paths

```rust
let users = firebase.at("users");
let user_ref = firebase.at("users").at(user_id);
```

Avoid hand-building `https://.../users/{id}.json` URLs in application code.

## Reads

Read one record:

```rust
let user = firebase.at("users").at(user_id).get::<User>().await?;
```

Read all records as Firebase key map:

```rust
use std::collections::HashMap;

let users = firebase.at("users").get::<HashMap<String, User>>().await?;
```

Read raw response:

```rust
let body = firebase.at("users").get_as_string().await?;
```

## Writes

Push with Firebase-generated key:

```rust
let user = User { name: "Ada".to_string() };
let response = firebase.at("users").set(&user).await?;
```

Write with a known key:

```rust
let user = User { name: "Ada".to_string() };
let mut users = firebase.at("users");
let response = users.set_with_key(user_id, &user).await?;
```

Update an existing record:

```rust
let patch = User { name: "Grace".to_string() };
let response = firebase.at("users").at(user_id).update(&patch).await?;
```

Delete a record:

```rust
let response = firebase.at("users").at(user_id).delete().await?;
```

## Query Parameters

```rust
let firebase = Firebase::new(&database_url)?
    .with_params()
    .order_by("name")
    .start_at(1)
    .equal_to(5)
    .finish();

let result = firebase.get::<serde_json::Value>().await?;
```

Use the result type expected by the database shape. Firebase REST query results are often maps keyed by record id.

## Realtime Events

Callback style:

```rust
let firebase = Firebase::new(&database_url)?.at("users");
let stream = firebase.with_realtime_events().expect("realtime events unavailable");

stream
    .listen(
        |event_type, data| {
            println!("Type: {:?} Data: {:?}", event_type, data);
        },
        |err| eprintln!("{err:?}"),
        false,
    )
    .await;
```

Async stream style:

```rust
use futures_util::StreamExt;

let firebase = Firebase::new(&database_url)?.at("users");
let stream = firebase
    .with_realtime_events()
    .expect("realtime events unavailable")
    .stream(true);

stream
    .for_each(|event| {
        match event {
            Ok((event_type, data)) => println!("{event_type:?} {data:?}"),
            Err(err) => eprintln!("{err:?}"),
        }
        futures_util::future::ready(())
    })
    .await;
```

Add `futures-util` when using the stream style directly.

## Error Handling Shape

In app code, prefer returning errors:

```rust
pub async fn load_user(firebase: &Firebase, user_id: &str) -> Result<User, firebase_rs::errors::RequestError> {
    firebase.at("users").at(user_id).get::<User>().await
}
```

If URL parsing can fail in setup, include that in the caller's existing error type with `?`, `anyhow`, or a local error enum.
