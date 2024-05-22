# Ludium Payment System
## About
ludium payment axum backend system.


## Local Execution
### quick dev
```
# test
cargo watch -q -c -w src/ -x run 

# test with debug
cargo watch -q -c -w tests/ -x  "test -q quick_dev -- --nocapture"
```

### local db 
```
# db setting
docker-compose -f ./docker-compose.localdb.yml up -d
```

### local test 
```
cargo test
```


## Deployment
todo