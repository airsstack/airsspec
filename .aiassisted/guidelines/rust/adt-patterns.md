# Algebraic Data Types (ADT) Patterns in Rust

This guide covers ADT patterns for type-safe, expressive Rust code.

---

## What are ADTs?

Algebraic Data Types are composite types formed by combining other types. In Rust:
- **Sum Types** — `enum` (one of several variants)
- **Product Types** — `struct` (combination of fields)

---

## §1. Sum Types with Enum (MANDATORY)

**Use `enum` for types that can be one of several variants:**

```rust
// ✅ CORRECT - Clear sum type
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Processing { transaction_id: String },
    Completed { amount: Money, timestamp: DateTime<Utc> },
    Failed { reason: String, code: u32 },
}

// ✅ CORRECT - Recursive types with Box
#[derive(Debug)]
pub enum Tree<T> {
    Empty,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}
```

**Rationale:**
- Exhaustive pattern matching enforced by compiler
- Clear enumeration of all possible states
- Type-safe state machines

---

## §2. Newtype Pattern (MANDATORY)

**Wrap primitive types to enforce type safety:**

```rust
// ✅ CORRECT - Newtypes prevent mixing up values
pub struct UserId(u64);
pub struct OrderId(u64);
pub struct Meters(f64);
pub struct Kilometers(f64);

// Compile error: can't pass UserId where OrderId expected
fn get_order(order_id: OrderId) -> Order { ... }

// ❌ FORBIDDEN - Bare primitives lose type safety
fn get_order(order_id: u64) -> Order { ... }  // Can pass any u64!
```

**When to use newtypes:**
- IDs (UserId, OrderId, SessionId)
- Units (Meters, Seconds, Bytes)
- Domain concepts (Email, Username, Price)

---

## §3. Newtype with Deref (OPTIONAL)

**Implement `Deref` for transparent access to inner value:**

```rust
use std::ops::Deref;

pub struct Email(String);

impl Deref for Email {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Usage: email.len(), email.contains("@")
```

**⚠️ Caution:** Only use Deref when the newtype IS-A the wrapped type semantically.

---

## §4. Type Aliases vs Newtypes

**Type aliases do NOT provide type safety:**

```rust
// ❌ NO type safety - aliases are interchangeable
type UserId = u64;
type OrderId = u64;

fn bad(user: UserId, order: OrderId) { ... }
bad(order_id, user_id);  // Compiles! Wrong order.

// ✅ Type safety with newtypes
pub struct UserId(u64);
pub struct OrderId(u64);

fn good(user: UserId, order: OrderId) { ... }
good(order_id, user_id);  // Compile error!
```

**Use type aliases ONLY for:**
- Reducing repetition of complex types
- Internal implementation shortcuts

```rust
// ✅ CORRECT - Alias for complex type
type Result<T> = std::result::Result<T, MyError>;
type Callback = Box<dyn Fn() + Send + 'static>;
```

---

## §5. Builder Pattern for Product Types

**Use builders for complex structs:**

```rust
#[derive(Debug)]
pub struct Config {
    host: String,
    port: u16,
    timeout: Duration,
}

#[derive(Default)]
pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<Duration>,
}

impl ConfigBuilder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    pub fn build(self) -> Result<Config, ConfigError> {
        Ok(Config {
            host: self.host.ok_or(ConfigError::MissingHost)?,
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
        })
    }
}
```

---

## §6. Phantom Types for Compile-Time State

**Use `PhantomData` to encode state in types:**

```rust
use std::marker::PhantomData;

// State markers (zero-sized types)
pub struct Draft;
pub struct Published;

pub struct Article<State> {
    title: String,
    content: String,
    _state: PhantomData<State>,
}

impl Article<Draft> {
    pub fn new(title: String) -> Self {
        Self { title, content: String::new(), _state: PhantomData }
    }
    
    pub fn publish(self) -> Article<Published> {
        Article {
            title: self.title,
            content: self.content,
            _state: PhantomData,
        }
    }
}

impl Article<Published> {
    // Only published articles can be shared
    pub fn share(&self) -> Url { ... }
}
```

---

## §7. Derive Macros (MANDATORY)

**Always derive common traits:**

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(u64);

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
}

// For serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: String,
}
```

**Standard derives:**
- `Debug` — Always (debugging)
- `Clone` — When copying needed
- `PartialEq`, `Eq` — For comparisons
- `Hash` — For HashMap keys
- `Default` — For builder patterns

---

## Summary

| Pattern | Use Case | Type Safety |
|---------|----------|-------------|
| `enum` | Sum types, state machines | ✅ |
| `struct` | Product types | ✅ |
| Newtype | Wrap primitives | ✅ |
| Type alias | Reduce repetition | ❌ |
| PhantomData | Compile-time state | ✅ |

---

## References

- [The Rust Book: Advanced Types](https://doc.rust-lang.org/book/ch20-03-advanced-types.html)
- [Algebraic Data Types in Four Languages](https://blog.softwaremill.com/algebraic-data-types-in-four-languages-858788043d4e)
