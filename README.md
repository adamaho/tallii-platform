# tallii-platform

The backend service for the tallii scoreboard app. Tallii is now an open-sourced local first platform for keeping score with your friends. Use the code as you wish. Fork it, clone it, submit PRs, all the things.

## Development

### Prerequisites

Rust - [Install](https://www.rust-lang.org/tools/install).
Docker Compose - [Install](https://docs.docker.com/compose/install/)

### Setting up the database

1. Install the sqlx-cli package
```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```
2. Run the database container
```
docker-compose up database
```
3. In a different terminal window, create the database
```
sqlx database create
```
4. Run the migrations
```
sqlx migrate run
```

### Running the server

In a different terminal window, run the server:

```
cargo run
```

