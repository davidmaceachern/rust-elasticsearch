# Rust Elasticsearch

Example api with elasticsearch.

- Pretty-printed Logging with `Femme`.
- Error Handling Pattern with `Thiserror`.

## Quick Start

Run Elasticsearch in a container

``` bash
./start.sh
```

Run the server.

``` bash
git clone git@github.com:davidmaceachern/rust-elasticsearch.git
cd /rust-elasticsearch
cargo run
```

## Build Docker image

```
docker build -t davidmaceachern/rust-elasticsearch .
```

## Run Docker image

```
docker run --rm -itp 8000:8000 davidmaceachern/rust-elasticsearch
```