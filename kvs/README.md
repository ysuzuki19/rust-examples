# Kvs

Redis like kvs with Rust.

# Feature

- Send Command via TCP connection
- Set value(String) for the key(String)
- Get value(String) for the key(String)

# Example of ...

- Async with `tokio`
- Multi Threading with `tokio`
- Tcp Stream with `tokio`
- RwLock
- enum with values
- `FromStr`/`ToStr` impl for enum
- pattern matching for enum
- Vec to Array
- Add method for `str`
- Custom Error with `thiserror` crate
- unit testing

# Run and send command

## Run

```bash
$ cargo run
```

## Send command

```bash
$ nc -v 0.0.0.0 8080
SET test 10
GET test
SET test 0
GET test
```

and you can use the server with any tcp client.
