# Use the base image
FROM rust:1.44.0 AS build

# Steps to build the rust binary
WORKDIR /src/rust-elastic-search

COPY . .

RUN cargo build --release

# The container the application will run in.
FROM ubuntu:18.04

COPY --from=build /src/rust-elastic-search/target/release/rust-elastic-search /usr/local/bin/rust-elastic-search

# Command to start the application when container is run.
CMD ["usr/local/bin/rust-elastic-search"]