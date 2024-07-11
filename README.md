# Ludium Reward Payment System
## About
Ludium Reward Payment is an Axum-based backend system designed to manage off-chain transactions and APIs, also serving as a on-chain relayer system. 

### off-chain things
The system manages reward payment transaction records and handles various APIs to facilitate smooth operations. This includes tracking and recording transaction histories, ensuring data consistency, and providing necessary endpoints for interacting with the system.

### on-chain things
Ludium Reward Payment utilizes NEAR protocol's meta-transaction capabilities to facilitate efficient transaction processing. the system is designed with scalability in mind to support future integration with multi-chain networks, enabling seamless transactions across different chains.

### Project Structure - Hexagonal Architecture
> check the [ARCHITECTURE.md](https://github.com/Ludium-Official/ludium-world-payment/blob/main/ARCHITECTURE.md)

payment system use Hexagonal Architecture to achieve scalability and flexibility. This architecture allows us to: 
- Isolate the core logic of the application from external dependencies
- Facilitate easier testing and maintenance
- Support future integration with multi-chain


## Getting Started 
### Prerequisites
Ensure you have the following installed on your local machine:
- Rust (1.78)
- Docker Compose
- PostgreSQL (14)

### Installation
1. Clone the repository:
```sh
git clone https://github.com/Ludium-Official/ludium-world-payment.git
cd ludium-world-payment
```

2. Set up the environment variables by creating a .env file:
```sh
cp .env.example .env.local
```

3. Set up near multi account_keys 
> check the [near/pagoda-relayer-rs readme](https://github.com/near/pagoda-relayer-rs#:~:text=Multiple%20Key%20Generation%20%2D%20OPTIONAL%2C%20but%20recommended%20for%20high%20throughput%20to%20prevent%20nonce%20race%20conditions)

You will need to create and set your own multi account_keys in the `/account_keys` folder, and set the `relayer_account_id`, `keys_filename` to match the account you set in `config.toml`.


4. run app & db container 
```sh
cd docker-compose && docker-compose -f ./local.yml up -d

// or just `cargo run`
```
> check the [docker-compose](./docker-compose/)

### For Development
1. Start the application with hot-reloading for development:
```sh
cargo watch -q -c -w src/ -x run
```

2. To run the tests with hot-reloading:
```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture --ignored"
```

### db schema update (FYI)
if you want to set a new db table. you have to print db schema file
> Note: The payment system does not use `disel migration` directly. You can configure and use Diesel migrations if needed. Currently, in development, running `scripts/dev_initial` will automatically set up the database with mock data.

```sh
# diesel print-schema --database-url={db_url} > {print_path}
diesel print-schema --database-url=postgres://postgres:postgres@localhost:5432/temp_local > src/adapter/output/persistence/db/schema.rs
```

