# Ludium Reward Payment System
## About
Ludium Reward Payment is an Axum-based backend system designed to manage off-chain transactions and APIs, also serving as a on-chain relayer system. 

### off-chain things
The system manages reward payment transaction records and handles various APIs to facilitate smooth operations. This includes tracking and recording transaction histories, ensuring data consistency, and providing necessary endpoints for interacting with the system.

### on-chain things
Ludium Reward Payment utilizes NEAR protocol's meta-transaction capabilities to facilitate efficient transaction processing. the system is designed with scalability in mind to support future integration with multi-chain networks, enabling seamless transactions across different chains.

### Project Structure - Hexagonal Architecture
> check the [ARCHITECTURE.md](https://github.com/Ludium-Official/ludium-world-payment/blob/main/README.md)

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

3. run app & db container 
```sh
docker-compose -f ./docker-compose.local.yml up -d
```

### Running the Application 
Start the application with hot-reloading for development:
```sh
cargo watch -q -c -w src/ -x run
```

### Running Tests
To run the tests with hot-reloading:
```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev2 -- --nocapture"
```

### local test 
```
cargo test
```


### db schema update (FYI)
if you want to set a new db table. you have to print db schema file
> Note: The payment system does not use `disel migration` directly. You can configure and use Diesel migrations if needed. Currently, in development, running `scripts/dev_initial` will automatically set up the database with mock data.

```sh
# diesel print-schema --database-url={db_url} > {print_path}
diesel print-schema --database-url=postgres://postgres:postgres@localhost:5432/ludium_local > src/adapter/output/persistence/db/schema.rs
```

### MultipleKey Generation for On-chain Relayer signer
check the [near/pagoda-relayer-rs readme](https://github.com/near/pagoda-relayer-rs#:~:text=Multiple%20Key%20Generation%20%2D%20OPTIONAL%2C%20but%20recommended%20for%20high%20throughput%20to%20prevent%20nonce%20race%20conditions)


## Deployment
todo

## License
todo 

