# About Payment System Architecture 
![Hexagonal Architecture](img/hexagonal-architecture.png)

## Inside World
### Domain(Entity)
At the core of the hexagonal architecture is the domain layer, which contains the application's business logic and rules. This layer is highly cohesive and independent of external systems or frameworks. It contains pure business logic and only allows changes related to business requirements.


```rust
// Example of a Domain entity 
#[derive(Debug, Clone)]
pub struct Payment {
    pub id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub status: PaymentStatus,
}

#[derive(Debug, Clone)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}
```

### Usecase
The Usecase layer handles what a Domain can do. It coordinates the flow of data to and from the Domain and external layers (Adapter, Port). 

```rust 
// Example of a Usecase in Rust
pub struct ProcessPayment {
    pub payment_repository: Box<dyn PaymentRepository>,
}

impl ProcessPayment {
    pub async fn execute(&self, payment: Payment) -> Result<(), PaymentError> {
        // Business logic to process a payment
    }
}
```

## Outside World
### Port
n interface or abstraction layer that defines how a domain interacts with the outside world. All communication with the outside world is done through Ports. Depending on how they interact with external systems, they are categorized into Inputs and Outputs. 

#### Input
Inputs are external systems coming into the system, such as API requests or the CLI. 
```rust
// Input Port (like CQRS pattern)
#[async_trait]
pub trait PaymentCommand {
    async fn create_payment(&self, create_payment_payload: CreatePaymentPayload) -> Result<PaymentResponse>;
}
```

#### Output
Output serves to interact with external systems, such as a DB or event messaging system. 
```rust
// Output Port
#[async_trait]
pub trait PaymentRepository {
    async fn save(&self, create_payment_payload: CreatePaymentPayload) -> Result<PaymentResponse>;
}
```

The input side handles the external interactions that come into the payment system. This includes receiving API calls or commands.

### Adapter
An Adapter is an implementation of a Port. It converts data and requests from external systems into a form that Domain can understand and process.

```rust
// Input Adapter 
impl PaymentCommand for PaymentCommandImpl {
        async fn create_payment(&self, create_payment_payload: CreatePaymentPayload) -> Result<PaymentResponse> {
        // Logic to process data through abstracted Domain implementations (usecase)

        Ok(Json(paymentResponse))
    }
}
```

```rust
// Output Adapter
#[async_trait]
impl PaymentRepository for PaymentRepositoryImpl {
    async fn save(&self, create_payment_payload: CreatePaymentPayload) -> Result<PaymentResponse> {
        // Logic for saving payment's data in the DB 

        Ok(Json(paymentResponse))
    }
}
```

## Resources
- Damir Svrtan and Sergii Makagon, [Ready for changes with Hexagonal Architecture](https://netflixtechblog.com/ready-for-changes-with-hexagonal-architecture-b315ec967749), Neflix Tech Blog