[![Logo](./docs/logo-landscape.png)](#)

# inmemory-cluster

⚠️ DISCLAIMER | This is a hobby project and not intended for production use. Use at your own risk!

## Motivation

inmemory-cluster is a simple yet powerful implementation of a distributed key-value store. Inspired by Longhorn, it offers an easy way to store values in memory across nodes, using a lightweight and friendly HTTP-based API.

This project has been a personal learning experience, helping me explore Rust, TCP networking, and distributed systems. For added fun, I've also implemented basic Redis API compatibility.

It is designed to be Kubernetes native

## Environment variables

- `ENV`: production, development
- `HOSTNAME`: Name to give to this instance. If not provided, it will default to the hostname.
- `PORT_HTTP`: Port to start HTTP server. Used for public API.
- `PORT_TCP`: Port to start TCP server. Used for internal API and redis API.

### HTTP API

#### Add a value

```bash
curl \
    -H "Content-type: application/json" \
    -X POST http://localhost:8999/data \
    -d '{ "key": "toto10", "value": "tata" }'
```

#### Read a value

```bash
curl \
    -H "Content-type: application/json" \
    -X http://localhost:8999/data/toto10 # toto10 is the key
```

#### Read stats about cluster

```
curl http://localhost:8999/stats
```

#### Test ingestion

```bash
seq 0 200 | xargs -n1 -P2 -I{} \
curl -s \
    -H "Content-type: application/json" \
    -X POST http://localhost:8999/data \
    -d "{\"key\": \"key{}\", \"value\": \"tata\"}"
```

### Redis API

This cluster is compatible with REDIS API

```bash
redis-cli -p 8888 # must be TCP port
```

Commands implemented:
- [x] CLIENT SETINFO LIB-NAME
- [x] CLIENT SETINFO LIB-VER
- [x] FLUSHALL
- [x] GET
- [x] SET

## Learning rust

Every value in Rust has an owner and a value can only have one owner at a time. When the variable is the owner of the assigned value.

#### Sources

- https://medium.com/@hizacharylee/what-is-clone-in-rust-and-when-to-use-it-49d3d3c91e79
