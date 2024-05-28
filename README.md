# Ludium Reward Payment System
## About
Ludium Reward Payment is an Axum backend system for off-chain transactions and an on-chain relayer system.

## Project Structure
check the [ARCHITECTURE.md](https://github.com/Ludium-Official/ludium-world-payment/blob/main/README.md)


## Local Execution
### quick dev
```
# run server with hotreload
cargo watch -q -c -w src/ -x run 

# run quickdev with hotreload
cargo watch -q -c -w tests/ -x  "test -q quick_dev2 -- --nocapture"
```

### local db 
```
# db setting
docker-compose -f ./docker-compose.localdb.yml up -d
```

### create db schema
```
# diesel print-schema --database-url={db_url} > {print_path}
diesel print-schema --database-url=postgres://postgres:postgres@localhost:5432/ludium_local > src/adapter/output/persistence/db/schema.rs
```

### local test 
```
cargo test
```


## Deployment
todo