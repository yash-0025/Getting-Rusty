# 📖 Volume 03: Custom Types, Enums & Algebraic Data Types
## 🇮🇳 Sampoorna Rust Grantha — Tritiya Adhyaya (Chapter 3)

> **Maha-Uddeshya (Mission):**  
> Is chapter ka uddeshya Rust ke type system ki core foundation — **Structs, Enums, Algebraic Data Types (ADT), Memory Layouts (`#[repr]`), Zero-Sized Types (ZST), Null Pointer Optimization (NPO), Recursive Types, Arbitrary Self Types, `#[non_exhaustive]` Semver Protection, Memory Offsets (`offset_of!`), `Option::take()` under `&mut self`, Boolean Blindness Elimination, The Builder Pattern, aur Exhaustive Pattern Matching** ko bare-metal level par master karna hai.  
> C++ ya Java me agar koi invalid state banani ho, toh developer runtime exception throw karta hai. Rust ka philosophy hai: **"Make Illegal States Unrepresentable"** (Type system ko aisa mathematically tight design karo ki invalid state ka code compile hi na ho sake!).  
> Is chapter ko master karne ke baad tum domain models ko hardware-aligned aur zero-bug tareeqe se architecture karna seekh jaoge! 🦀⚡

---

## 🧭 Table of Contents
1. [Product Types vs Sum Types: The Algebraic Type Theory](#1-product-types-vs-sum-types-the-algebraic-type-theory)
2. [Struct Architecture: Named-Field Structs & Compiler Field Reordering](#2-struct-architecture-named-field-structs--compiler-field-reordering)
3. [Field Init Shorthand & Struct Update Syntax (`..base`)](#3-field-init-shorthand--struct-update-syntax-base)
4. [Visibility & Encapsulation: Struct Fields vs Enum Variants & `#[non_exhaustive]`](#4-visibility--encapsulation-struct-fields-vs-enum-variants--non_exhaustive)
5. [Tuple Structs & The Newtype Pattern: Type Safety Without Cost](#5-tuple-structs--the-newtype-pattern-type-safety-without-cost)
6. [Unit Structs: Zero-Sized Types (ZST) & The Typestate Pattern](#6-unit-structs-zero-sized-types-zst--the-typestate-pattern)
7. [Memory Layout Representations: `#[repr(Rust)]`, `#[repr(C)]`, `#[repr(packed)]`, `#[repr(align)]`, `#[repr(transparent)]`](#7-memory-layout-representations-reprrust-reprc-reprpacked-repralign-reprtransparent)
8. [Hardware Field Offsets: `std::mem::offset_of!` (Rust 1.77+)](#8-hardware-field-offsets-stdmemoffset_of-rust-177)
9. [Methods, Associated Functions & Advanced Self Receivers (`Box<Self>`, `Arc<Self>`)](#9-methods-associated-functions--advanced-self-receivers-boxself-arcself)
10. [Enums as Algebraic Data Types: Tagged Unions & Memory Footprints](#10-enums-as-algebraic-data-types-tagged-unions--memory-footprints)
11. [Recursive Enums & The Infinite Size Problem: Indirection with `Box<T>`](#11-recursive-enums--the-infinite-size-problem-indirection-with-boxt)
12. [Empty/Uninhabited Enums: `enum Void {}` & Never Types](#12-emptyuninhabited-enums-enum-void--never-types)
13. [The Null Pointer Optimization (NPO): 8-Byte `Option<&T>` Magic](#13-the-null-pointer-optimization-npo-8-byte-optiont-magic)
14. [Enum Discriminant Inspection: `std::mem::discriminant`](#14-enum-discriminant-inspection-stdmemdiscriminant)
15. [Standard Enums Masterclass: `Option<T>`, `Result<T, E>` & `ControlFlow`](#15-standard-enums-masterclass-optiont-resultt-e--controlflow)
16. [The `Option::take()` & `replace()` Secret Under `&mut self`](#16-the-optiontake--replace-secret-under-mut-self)
17. [Boolean Blindness: Replacing `bool` with Domain Enums](#17-boolean-blindness-replacing-bool-with-domain-enums)
18. [The Builder Pattern: Safe Construction of Complex Structs](#18-the-builder-pattern-safe-construction-of-complex-structs)
19. [Advanced Pattern Matching: Refutable vs Irrefutable Patterns](#19-advanced-pattern-matching-refutable-vs-irrefutable-patterns)
20. [Deep Destructuring, Slicing Patterns (`[first, .., last]`), `@` Bindings & Match Guards](#20-deep-destructuring-slicing-patterns-first--last--bindings--match-guards)
21. ["Why, When, Where, How & Why This Not That" 15-Point Systems Matrix](#21-why-when-where-how--why-this-not-that-15-point-systems-matrix)
22. [Master Working Code & Line-by-Line Syntax Walkthrough](#22-master-working-code--line-by-line-syntax-walkthrough)

---

## 1. Product Types vs Sum Types: The Algebraic Type Theory

Computer science me data types ko do mathematical categories me divide kiya ja sakta hai:

```
┌────────────────────────────────────────────────────────────────────────┐
│                   ALGEBRAIC DATA TYPES (ADT) MATRIX                    │
├──────────────────────────────────┬─────────────────────────────────────┤
│      PRODUCT TYPES (Structs)     │         SUM TYPES (Enums)           │
├──────────────────────────────────┼─────────────────────────────────────┤
│ Possible States = TypeA × TypeB  │ Possible States = TypeA + TypeB     │
│ Dono fields EK SATH exist karte  │ Sirf EK variant AT A TIME exist     │
│ hain (AND relationship).         │ karta hai (OR relationship).        │
│ Example: `struct Point(u8, bool)`│ Example: `enum State { On, Off }`   │
│ Total states: 256 × 2 = 512.     │ Total states: 1 + 1 = 2.            │
└──────────────────────────────────┴─────────────────────────────────────┘
```

- **Product Types (Structs):** Jab tumhe multiple pieces of data ko ek bundle me pack karna ho. Har field memory me apni independent jagah leta hai.
- **Sum Types (Enums):** Jab ek variable multiple roop me se koi **ek hi roop** le sakta ho. Memory me sirf largest variant ki space allocate hoti hai + ek chhota discriminant tag.

---

## 2. Struct Architecture: Named-Field Structs & Compiler Field Reordering

### 🏗️ Named-Field Struct Anatomy:
```rust
struct NetworkPacket {
    header: u8,      // 1 Byte
    payload_len: u32,// 4 Bytes
    flags: u8,       // 1 Byte
    checksum: u64,   // 8 Bytes
}
```

### 🧠 The Secret: Compiler Field Reordering (`#[repr(Rust)]`)
C language me structs me fields exact declaration order me memory me pack hote hain, jisse memory alignment holes (padding waste) create hoti hain:
```
C Layout (Wasteful 24 Bytes):
[header: 1B] [pad: 3B] [payload_len: 4B] [flags: 1B] [pad: 7B] [checksum: 8B] = 24 Bytes! (10 Bytes wasted!)
```

Rust ka default compiler layout (`#[repr(Rust)]`) fields ko hardware alignment ke hisaab se **automatically reorder** kar deta hai bina tumhari declaration change kiye:
```
Rust Optimized Layout (Only 16 Bytes!):
[checksum: 8B] [payload_len: 4B] [header: 1B] [flags: 1B] [pad: 2B] = 16 Bytes!
(8 Bytes RAM saved per packet, zero cache line waste!)
```

---

## 3. Field Init Shorthand & Struct Update Syntax (`..base`)

### ⚡ 1. Field Init Shorthand:
Agar local variable ka naam struct ke field se match karta hai, toh redundant typing ki zaroorat nahi:
```rust
let username = String::from("yash");
let active = true;

// Redundant: User { username: username, active: active }
let user = User { username, active }; // Clean & idiomatic!
```

### 🔄 2. Struct Update Syntax (`..base`):
Jab tumhe kisi existing struct ke base par naya struct banana ho aur sirf 1-2 fields change karne hon:
```rust
let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@rust.org"),
    login_count: 1,
    active: true,
};

let user2 = User {
    username: String::from("bob"),
    email: String::from("bob@rust.org"),
    ..user1 // Baaki sabhi fields user1 se copy/move ho jayenge!
};
```
*⚠️ Partial Move Warning:* Agar `user1` ke non-Copy fields (jaise `email`) update syntax me move ho gaye, toh `user1` partially moved ho jayega aur uske moved fields access nahi kiye ja sakte!

---

## 4. Visibility & Encapsulation: Struct Fields vs Enum Variants & `#[non_exhaustive]`

### 🔒 1. Struct Field Privacy:
Structs me encapsulation strict hota hai:
- Agar struct `pub struct Config` hai, toh uske fields by default **PRIVATE** rehte hain!
- Bahar ke modules direct `config.timeout` access nahi kar sakte jab tak field explicit `pub timeout: u32` na ho.
- **Why this matters?** Private fields maintain domain invariants. Agar field private hai, toh caller sirf tumhare constructor `Config::new()` aur validation methods ke through data set kar sakta hai!
- Fine-grained visibility: `pub(crate)` (visible inside current package), `pub(super)` (visible to parent module).

### 🌐 2. Enum Variant Visibility (All-or-Nothing):
Enums me visibility all-or-nothing hoti hai:
- Agar enum `pub enum Status` hai, toh uske **sabhi variants automatically `pub`** ho jaate hain! Individual variants par `pub` ya private lagana allowed nahi hai.

### 🛡️ 3. `#[non_exhaustive]` Attribute (Semver API Shield):
Socho tum ek library author ho aur tumne public enum ya struct publish kiya:
```rust
#[non_exhaustive]
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    Timeout,
}
```
`#[non_exhaustive]` compiler ko enforce karta hai:
- Bahar ke crates is enum ko exhaustively `match` nahi kar sakte bina wildcard `_ => {}` arm likhe!
- Future me jab tum naya variant `RateLimited` add karoge, toh external consumers ka code break **NAHI** hoga! Semver breaking change avoid ho jata hai!

---

## 5. Tuple Structs & The Newtype Pattern: Type Safety Without Cost

### 📦 1. Tuple Structs:
Positional indices (`.0`, `.1`) use karte hain:
```rust
struct Color(u8, u8, u8);
let black = Color(0, 0, 0);
println!("Red channel: {}", black.0);
```

### 🛡️ 2. The Newtype Pattern (Zero-Cost Domain Safety):
Socho tum ek financial ya token transaction system bana rahe ho:
```rust
// ❌ Naive Approach: Primitive Obsession
fn transfer(from_account: u64, to_account: u64, amount: u64) { ... }

// Bug: Junior dev ne galti se account IDs swap kar diye!
transfer(to_account, from_account, amount); // 💥 Compiles fine, lekin galat account drain ho gaya!
```

### 💡 The Idiomatic Rust Solution: The Newtype Pattern
```rust
struct SenderAccount(pub u64);
struct ReceiverAccount(pub u64);

fn transfer(from: SenderAccount, to: ReceiverAccount, amount: u64) { ... }

// transfer(to, from, amount); 
// 🚨 COMPILE ERROR: expected `SenderAccount`, found `ReceiverAccount`!
```
- **Zero Runtime Cost:** Machine code assembly level par `SenderAccount(100)` compile hokar raw `100` CPU register value ban jata hai. Zero memory overhead, zero CPU cycles, 100% compile-time safety!

---

## 6. Unit Structs: Zero-Sized Types (ZST) & The Typestate Pattern

### 🏷️ 1. Unit Structs (0 Bytes in RAM!):
```rust
struct Disconnected;
struct Connected;
struct Authenticated;
```
- `std::mem::size_of::<Disconnected>() == 0`!
- Compiler ZST ke liye hardware me 1 byte bhi allocate nahi karta.

### 🏛️ 2. The Typestate Pattern: Making Misuse Literally Impossible to Compile!
Typestate pattern state machine ko **Type System** ke andar encode kar deta hai:

```rust
// Typestate Pattern: Connection state is encoded in the TYPE!
struct Client<State> {
    socket_fd: i32,
    state: std::marker::PhantomData<State>, // Zero-cost state tag
}

impl Client<Disconnected> {
    pub fn new() -> Self {
        Client { socket_fd: -1, state: std::marker::PhantomData }
    }

    pub fn connect(self) -> Client<Connected> {
        println!("Connecting socket...");
        Client { socket_fd: 42, state: std::marker::PhantomData }
    }
}

impl Client<Connected> {
    pub fn send_data(&self, msg: &str) {
        println!("Sending data: {}", msg);
    }
}

// Usage:
let client = Client::new(); // Type: Client<Disconnected>
// client.send_data("Hello"); // 💥 COMPILE ERROR: no method `send_data` on `Client<Disconnected>`!

let connected_client = client.connect(); // Transitions to Client<Connected>
connected_client.send_data("Hello, Solana!"); // ✅ Perfectly Valid!
```

---

## 7. Memory Layout Representations: `#[repr(Rust)]`, `#[repr(C)]`, `#[repr(packed)]`, `#[repr(align)]`, `#[repr(transparent)]`

| Attribute | Systems Role & Memory Layout | Use Case |
|---|---|---|
| `#[repr(Rust)]` | **Default.** Compiler field reordering allow karta hai padding kam karne ke liye. | Pure Rust internal applications. |
| `#[repr(C)]` | C-ABI compatible layout. Fields exact declaration order me pack hote hain jaise C compiler karta hai. | **C FFI, Linux Kernel Drivers, Foreign APIs.** |
| `#[repr(packed)]` | Zero padding holes. Sabhi fields ek doosre se sata kar pack hote hain. | Network packet binary headers, Hardware protocols. *(⚠️ Dereferencing unaligned packed fields is unsafe!)* |
| `#[repr(align(N))]` | Type ka memory alignment force karta hai to power-of-two (e.g. `align(64)`). | **CPU Cache-Line (64B) false-sharing avoidance, SIMD AVX registers.** |
| `#[repr(transparent)]` | Single field struct ka exact same memory representation aur ABI hota hai jo uske inner field ka hai. | **Newtype wrappers passing across FFI boundaries.** |

---

## 8. Hardware Field Offsets: `std::mem::offset_of!` (Rust 1.77+)

Rust 1.77 me standardize hua `std::mem::offset_of!` macro kisi struct ke field ka exact byte offset calculate karta hai:

```rust
#[repr(C)]
struct Header {
    version: u8,   // Offset: 0
    // Padding: 3 bytes
    length: u32,   // Offset: 4
    checksum: u64, // Offset: 8
}

let len_offset = std::mem::offset_of!(Header, length);
assert_eq!(len_offset, 4);
```
Low-level driver programming, serialization engines aur bare-metal hardware packet parsers me ye invaluable tool hai.

---

## 9. Methods, Associated Functions & Advanced Self Receivers (`Box<Self>`, `Arc<Self>`)

Rust me `impl` block ke andar functions do types ke hote hain:
1. **Associated Functions (Static Methods):** Inme koi `self` parameter nahi hota (e.g. `Self::new()`).
2. **Methods:** Inme pehla parameter `self` hota hai.

### 🎭 The Self Receiver Spectrum:
- `&self`: Shared immutable borrow. Caller instance retain karta hai, read-only inspection.
- `&mut self`: Exclusive mutable borrow. In-place state modification.
- `self`: Consuming/By-Value. Instance move ho jata hai aur method ke baad destroy ho jata hai (Builder pattern terminal methods).
- `self: Box<Self>`: **Arbitrary Self Receiver!** Method tabhi call ho sakta hai jab struct heap-allocated `Box` ke andar ho!
- `self: Arc<Self>`: Method tabhi call ho sakta hai jab struct multi-threaded atomic pointer ke andar ho (Concurrency dispatch pipelines).

```rust
struct TaskNode {
    id: u64,
}

impl TaskNode {
    // Arbitrary self receiver: takes Box<Self>
    pub fn execute_boxed(self: Box<Self>) {
        println!("Executing heap-allocated task ID: {}", self.id);
    }
}
```

---

## 10. Enums as Algebraic Data Types: Tagged Unions & Memory Footprints

Rust ke Enums **Algebraic Data Types (Sum Types / Tagged Unions)** hote hain jinke har variant ke sath data attach ho sakta hai:

```rust
enum NetworkEvent {
    Connected(SocketAddr),                 // Tuple payload
    Message { payload: Vec<u8>, id: u64 }, // Struct payload
    Disconnected,                          // Unit payload
}
```

### 🔬 Memory Footprint of Enums:
$$\text{Size of Enum} = \text{Discriminant Tag (1-8 Bytes)} + \text{Size of Largest Variant} + \text{Padding}$$

```
┌────────────────────────────────────────────────────────┐
│                   ENUM MEMORY LAYOUT                   │
├─────────────────────┬──────────────────────────────────┤
│  Discriminant Tag   │     Largest Variant Payload      │
│   (e.g., 0x01)      │     (Union of all variant data)  │
└─────────────────────┴──────────────────────────────────┘
```

---

## 11. Recursive Enums & The Infinite Size Problem: Indirection with `Box<T>`

Socho tum ek recursive binary tree ya linked list banana chahte ho:

```rust
// ❌ COMPILE ERROR: recursive type `ListNode` has infinite size!
enum ListNode {
    Cons(i32, ListNode), // ListNode ke andar ListNode!
    Nil,
}
```

### 🧠 Kyun Compiler Ise Reject Karta Hai?
Rust compiler ko compile time par har type ka exact fixed byte size calculate karna hota hai taaki stack frame allocate ho sake.  
`ListNode` ka size hoga: `4 bytes + Size(ListNode) = 4 + 4 + 4 + ... = Infinite Bytes!`

### 💡 The Solution: Indirection via `Box<T>`
```rust
enum ListNode {
    Cons(i32, Box<ListNode>), // Box<T> has a fixed size of 8 Bytes!
    Nil,
}
```
Stack par `Box` ka pointer sirf **8 bytes** leta hai, chahe heap par recursive chain kitni bhi lambi ho!

---

## 12. Empty/Uninhabited Enums: `enum Void {}` & Never Types

Rust me ek aisa enum define kiya ja sakta hai jisme **zero variants** hote hain:

```rust
pub enum Void {}
```
- `std::mem::size_of::<Void>() == 0`.
- **Inhabited vs Uninhabited:** Is enum ka koi instance banana **literally impossible** hai kyunki koi variant exist hi nahi karta!
- **Where is it used?** Jab koi generic type failure case ko impossible banana chahta ho:
  `Result<T, Void>` guarantee karta hai ki ye operation kabhi fail ho hi nahi sakta!

---

## 13. The Null Pointer Optimization (NPO): 8-Byte `Option<&T>` Magic

Tony Hoare ne `null` reference ko apna *"Billion Dollar Mistake"* kaha tha.  
Rust me `null` exist hi nahi karta! Agar value missing ho sakti hai, toh hum `Option<T>` use karte hain:

```rust
std::mem::size_of::<&String>()         == 8 Bytes
std::mem::size_of::<Option<&String>>() == 8 Bytes! (ZERO OVERHEAD NULL SAFETY!)
```

### 🎩 How NPO Works Under the Hood:
- Rust me references (`&T`) aur non-null smart pointers (`Box<T>`, `NonNull<T>`) kabhi `0x0` address nahi ho sakte.
- Compiler `0x0` null address ko silently `None` state ke roop me encode kar deta hai!
- C me `char*` null check karne me 8 bytes leta hai, aur Rust me 100% type-safe `Option<&T>` bhi exact **8 bytes** leta hai with zero discriminator overhead!

---

## 14. Enum Discriminant Inspection: `std::mem::discriminant`

Kabhi-kabhi tumhe do enum instances ko compare karna hota hai ye dekhne ke liye ki kya wo **same variant** hain bina unke internal payloads ko inspect kiye:

```rust
use std::mem::discriminant;

let ev1 = NetworkEvent::Disconnected;
let ev2 = NetworkEvent::Disconnected;

if discriminant(&ev1) == discriminant(&ev2) {
    println!("Both events are of the same variant type!");
}
```
`std::mem::discriminant` internal tag ko extract karta hai jo `PartialEq` aur `Hash` implement karta hai.

---

## 15. Standard Enums Masterclass: `Option<T>`, `Result<T, E>` & `ControlFlow`

### 1. `Option<T>` Combinators:
- `.map(|x| x * 2)`: Transform inner value.
- `.and_then(|x| fetch(x))`: FlatMap to avoid `Option<Option<U>>`.
- `.unwrap_or(default)`: Safe fallback default.
- `.unwrap_or_else(|| compute())`: Lazy fallback closure.
- `.ok_or(Err)`: Convert `Option<T>` into `Result<T, E>`.
- `.transpose()`: Converts `Option<Result<T, E>>` into `Result<Option<T>, E>`!

### 2. `Result<T, E>` Combinators:
- `.map_err(|e| CustomErr::from(e))`
- `.and_then(|val| next_op(val))`
- `.is_ok()`, `.is_err()`

### 3. `ControlFlow<B, C>` (Rust 1.55+):
Recursive traversal patterns me boolean flag hacks ko replace karta hai:
```rust
use std::ops::ControlFlow;

fn evaluate(item: i32) -> ControlFlow<i32, ()> {
    if item == 42 {
        ControlFlow::Break(item) // Stop traversal!
    } else {
        ControlFlow::Continue(()) // Keep going!
    }
}
```

---

## 16. The `Option::take()` & `replace()` Secret Under `&mut self`

Ek bohot bada senior gotcha: Jab tumhare paas `&mut self` reference hota hai, toh tum struct ke kisi field se value **Move nahi kar sakte** kyunki Rust struct ko uninitialized nahi chhodne deta:

```rust
struct SessionManager {
    active_token: Option<String>,
}

impl SessionManager {
    // ❌ WRONG: Cannot move out of `self.active_token` behind a mutable reference!
    // pub fn extract_token(&mut self) -> String { self.active_token.unwrap() }

    // ✅ SENIOR IDIOMATIC SOLUTION: Option::take()
    pub fn extract_token(&mut self) -> Option<String> {
        self.active_token.take() // Takes the value out, leaving `None` in its place!
    }
}
```
`.take()` borrow checker rules ko violate kiye bina value nikal leta hai aur field me safely `None` chhod deta hai!

---

## 17. Boolean Blindness: Replacing `bool` with Domain Enums

### 🚨 The Problem: Boolean Blindness
```rust
// Boolean Blindness: Ye true aur false kya kar rahe hain?!
configure_database(true, false, true);
```
Caller code padh kar kisi ko nahi pata chalta ki kaun sa flag kiske liye hai.

### 💡 The Solution: Domain Enums
```rust
enum CacheStrategy { Enabled, Disabled }
enum LoggingLevel { Verbose, Silent }
enum AutoReconnect { Yes, No }

configure_database(
    CacheStrategy::Enabled,
    LoggingLevel::Silent,
    AutoReconnect::Yes,
);
```
Code instantly self-documenting aur type-safe ban jata hai!

---

## 18. The Builder Pattern: Safe Construction of Complex Structs

Jab kisi struct me 10+ fields hon jinme se kuch optional hon:

```rust
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub max_connections: u32,
}

pub struct ServerConfigBuilder {
    host: String,
    port: u16,
    tls: bool,
    max_connections: u32,
}

impl ServerConfigBuilder {
    pub fn new(host: &str) -> Self {
        ServerConfigBuilder {
            host: host.to_string(),
            port: 8080,
            tls: false,
            max_connections: 1000,
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn tls(mut self, tls: bool) -> Self {
        self.tls = tls;
        self
    }

    pub fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host,
            port: self.port,
            tls: self.tls,
            max_connections: self.max_connections,
        }
    }
}
```

---

## 19. Advanced Pattern Matching: Refutable vs Irrefutable Patterns

1. **Irrefutable Patterns:** Jo **hamesha succeed karte hain**.
   - `let x = 5;`
   - `let Point { x, y } = pt;`
   - Normal `let` statements me allowed hain.
2. **Refutable Patterns:** Jo **fail ho sakte hain**.
   - `Some(v)`, `Ok(v)`, `1..=10`.
   - Normal `let` me compiler error dete hain (`let Some(x) = opt;` fails!).
   - Inhe `if let`, `let ... else`, ya `match` me handle kiya jata hai.

---

## 20. Deep Destructuring, Slicing Patterns (`[first, .., last]`), `@` Bindings & Match Guards

```rust
// 1. Nested Struct & Enum Destructuring with Match Guards:
match event {
    NetworkEvent::Message { payload, id } if payload.is_empty() => {
        println!("Empty message dropped for ID: {}", id);
    }
    NetworkEvent::Connected(addr) => println!("Connected: {}", addr),
    NetworkEvent::Disconnected => println!("Clean disconnect"),
    _ => {},
}

// 2. Slice Pattern Matching:
let slots = [100, 101, 102, 103, 104];
match slots {
    [genesis, .., tip] => println!("Genesis: {}, Tip: {}", genesis, tip),
}

// 3. Subpattern Binding with @:
match user_id {
    vip @ 1..=50 => println!("VIP Clearance for ID: {}", vip),
    standard @ 51..=1000 => println!("Standard Clearance for ID: {}", standard),
    _ => println!("External Guest"),
}
```

---

## 21. "Why, When, Where, How & Why This Not That" 15-Point Systems Matrix

| Scenario / Choice | Kya Chunein? | Kya Reject Kiya & Kyun? | Engineering Reason & Hardware Impact |
|---|---|---|---|
| **Distinct Identity Types** | Newtype `struct UserId(u64)` | Type alias `type UserId = u64` | Type alias sirf synonym hai (swapped args compile ho jate hain); Newtype compile error deta hai with 0 runtime cost. |
| **Mutually Exclusive States** | `enum State { Off, On(u32) }` | `struct State { is_on: bool, val: u32 }` | Struct me invalid combination (`is_on: false, val: 99`) represent ho sakta hai; Enum invalid state ko mathematically impossible banata hai. |
| **Missing Values** | `Option<T>` | Sentinel values (`-1`, `null`, `""`) | Sentinel values boundary condition bugs cause karte hain; `Option<T>` compiler exhaustiveness check enforce karta hai. |
| **C-ABI Hardware Drivers** | `#[repr(C)]` | Default `#[repr(Rust)]` | `#[repr(Rust)]` fields reorder karta hai jisse C-struct byte alignment mismatch ho jata hai aur kernel crash hota hai. |
| **Network Binary Packets** | `#[repr(packed)]` | Padded structs | `packed` struct memory padding holes zero rakhta hai jisse raw bytes direct socket par stream ho sakte hain. |
| **Zero-Cost State Machines** | Unit Structs (ZST Typestate) | Runtime boolean flags | ZST Typestate galat method calls ko compile time par reject kar deta hai with 0 bytes RAM usage. |
| **Optional References** | `Option<&T>` | Raw pointer with null check | NPO ki wajah se `Option<&T>` exact 8 bytes ka hota hai aur 100% memory-safe rehta hai. |
| **Recursive Data Types** | `Box<T>` indirection | Inlined recursive struct | Direct recursion causes infinite size compilation error; `Box` fixed 8-byte pointer deta hai. |
| **Uninhabited Impossible States** | Empty Enum `enum Void {}` | Dummy structs | Empty enum cannot be instantiated; guarantees mathematically impossible code branches. |
| **Comparing Enum Variants** | `std::mem::discriminant()` | Manual pattern matching | Variant tags compare karne ka cleanest and fastest zero-payload extraction method. |
| **Moving Field Under `&mut self`**| `field.take()` | Unsafe pointer copy | `take()` safely leaves `None` in place, preventing invalid memory state while yielding owned data. |
| **Boolean Parameter Overload** | Domain Enums | Bare `bool` flags | Boolean Blindness eliminate karta hai; self-documenting code banata hai. |
| **Constructing Complex Structs** | The Builder Pattern | Long 10-arg constructors | Field ordering errors eliminate karta hai aur sensible default values support karta hai. |
| **Library Semver Extensibility** | `#[non_exhaustive]` | Plain public enums | Consumers ko wildcard handle karne par force karta hai; future variant additions me breaking change avoid hoti hai. |
| **Inspecting Struct Offsets** | `std::mem::offset_of!` | Pointer arithmetic hacks | Compile-time safe field offset inspection bina undefined behavior trigger kiye. |

---

## 22. Master Working Code & Line-by-Line Syntax Walkthrough

Chalo ek complete, runnable, production-quality program dekhte hain jo Volume 3 ke har ek deep concept ko demonstrate karta hai:

```rust
// File: rust_book_vol3_mastery.rs

use std::marker::PhantomData;
use std::ops::ControlFlow;
use std::mem::discriminant;

/// Complete demonstration of Structs, Enums, NPO, Typestates, Recursive Types, and Pattern Matching.
pub fn run_volume_3_mastery() {
    println!("=== 1. STRUCTS & NEWTYPE PATTERN ===");
    let uid = UserId(1001);
    let oid = OrderId(9999);
    println!("Type-safe IDs created: uid={}, oid={}", uid.0, oid.0);

    // Named Struct with Field Init Shorthand & Update Syntax
    let sku = String::from("RUST-BOOK-2026");
    let base_item = InventoryItem {
        sku,
        quantity: 50,
        in_stock: true,
    };

    let updated_item = InventoryItem {
        quantity: 45,
        ..base_item // Struct update syntax!
    };
    println!("Updated item sku: {}, qty: {}", updated_item.sku, updated_item.quantity);

    println!("\n=== 2. ZERO-SIZED TYPES (ZST) & TYPESTATE PATTERN ===");
    println!("Size of Unit Struct Marker: {} bytes", std::mem::size_of::<Disconnected>());
    
    // Typestate Connection Flow
    let client = DatabaseConnection::<Disconnected>::new();
    let connected_client = client.connect("postgres://localhost:5432");
    connected_client.query("SELECT * FROM bookmarks");

    println!("\n=== 3. ENUMS, NPO & RECURSIVE TYPES ===");
    println!("Size of raw &String: {} bytes", std::mem::size_of::<&String>());
    println!("Size of Option<&String>: {} bytes (NPO in Action!)", 
        std::mem::size_of::<Option<&String>>());

    // Recursive List with Box Indirection
    let list = RecursiveList::Cons(1, Box::new(RecursiveList::Cons(2, Box::new(RecursiveList::Nil))));
    println!("Recursive list head value: {}", list.head().unwrap_or(-1));

    // Discriminant comparison
    let ev1 = TransactionEvent::Pending;
    let ev2 = TransactionEvent::Pending;
    if discriminant(&ev1) == discriminant(&ev2) {
        println!("Discriminants matched: Both events are Pending!");
    }

    // Option::take under &mut self
    let mut session = SessionManager { active_token: Some(String::from("jwt_secret_token")) };
    let extracted = session.extract_token();
    println!("Extracted token: {:?}, Remaining in session: {:?}", extracted, session.active_token);

    println!("\n=== 4. THE BUILDER PATTERN ===");
    let server = ServerConfigBuilder::new("127.0.0.1")
        .port(9000)
        .tls(true)
        .build();
    println!("Server built: {}:{} (TLS: {})", server.host, server.port, server.tls);

    println!("\n=== 5. ADVANCED PATTERN MATCHING & SLICING ===");
    let block_headers = [100, 101, 102, 103, 104];
    match block_headers {
        [genesis, .., tip] => println!("Genesis slot: {}, Tip slot: {}", genesis, tip),
    }

    // ControlFlow demonstration
    let numbers = [5, 12, 42, 88];
    for n in numbers {
        if let ControlFlow::Break(val) = find_magic_number(n) {
            println!("Found magic number early and broke traversal: {}", val);
            break;
        }
    }
}

// --- DOMAIN TYPES & NEWTYPES ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderId(pub u64);

pub struct InventoryItem {
    pub sku: String,
    pub quantity: u32,
    pub in_stock: bool,
}

// --- TYPESTATE PATTERN IMPLEMENTATION ---
pub struct Disconnected;
pub struct Connected;

pub struct DatabaseConnection<State> {
    pub connection_str: String,
    pub state_marker: PhantomData<State>,
}

impl DatabaseConnection<Disconnected> {
    pub fn new() -> Self {
        DatabaseConnection {
            connection_str: String::new(),
            state_marker: PhantomData,
        }
    }

    pub fn connect(self, endpoint: &str) -> DatabaseConnection<Connected> {
        println!("Handshake successful with {}", endpoint);
        DatabaseConnection {
            connection_str: endpoint.to_string(),
            state_marker: PhantomData,
        }
    }
}

impl DatabaseConnection<Connected> {
    pub fn query(&self, sql: &str) {
        println!("Executing SQL on [{}]: {}", self.connection_str, sql);
    }
}

// --- RECURSIVE ENUM WITH BOX ---
pub enum RecursiveList {
    Cons(i32, Box<RecursiveList>),
    Nil,
}

impl RecursiveList {
    pub fn head(&self) -> Option<i32> {
        match self {
            RecursiveList::Cons(val, _) => Some(*val),
            RecursiveList::Nil => None,
        }
    }
}

// --- ENUMS AS ALGEBRAIC DATA TYPES WITH #[non_exhaustive] ---
#[non_exhaustive]
pub enum TransactionEvent {
    Pending,
    Committed { tx_hash: String, slot: u64 },
    Failed(String),
}

// --- SESSION MANAGER DEMONSTRATING OPTION::TAKE ---
pub struct SessionManager {
    pub active_token: Option<String>,
}

impl SessionManager {
    pub fn extract_token(&mut self) -> Option<String> {
        self.active_token.take() // Moves value out, leaves None behind safely!
    }
}

// --- BUILDER PATTERN ---
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub tls: bool,
}

pub struct ServerConfigBuilder {
    host: String,
    port: u16,
    tls: bool,
}

impl ServerConfigBuilder {
    pub fn new(host: &str) -> Self {
        ServerConfigBuilder { host: host.to_string(), port: 8080, tls: false }
    }
    pub fn port(mut self, port: u16) -> Self { self.port = port; self }
    pub fn tls(mut self, tls: bool) -> Self { self.tls = tls; self }
    pub fn build(self) -> ServerConfig {
        ServerConfig { host: self.host, port: self.port, tls: self.tls }
    }
}

fn find_magic_number(n: i32) -> ControlFlow<i32, ()> {
    if n == 42 { ControlFlow::Break(n) } else { ControlFlow::Continue(()) }
}
```

### 🔬 Line-by-Line Syntax & Engineering Walkthrough (Per Rule 11):

1. `pub struct UserId(pub u64);`:
   - Tuple struct definition. `pub` visibility allows external modules to construct `UserId(1001)`. Zero runtime overhead; compiles to raw `u64`.

2. `let updated_item = InventoryItem { quantity: 45, ..base_item };`:
   - `..base_item`: Struct update syntax. `sku` (String) is moved, `in_stock` (bool) is copied into `updated_item`.

3. `pub struct DatabaseConnection<State> { ... state_marker: PhantomData<State> }`:
   - Typestate container. `PhantomData<State>` compiles to 0 bytes, letting us bind compile-time states without memory cost.

4. `pub enum RecursiveList { Cons(i32, Box<RecursiveList>), Nil }`:
   - `Box<RecursiveList>`: Heap indirection pointer. Solves the infinite size recursion compilation error, locking the stack footprint to exactly 8 bytes for the pointer.

5. `#[non_exhaustive] pub enum TransactionEvent { ... }`:
   - Semver protection attribute. Forces downstream library consumers to include a wildcard `_ => {}` arm in their `match` expressions.

6. `self.active_token.take()`:
   - Critical senior idiom: Extracts the owned `String` from `self.active_token` behind an `&mut self` borrow, replacing it with `None` atomically.

7. `ServerConfigBuilder::new("127.0.0.1").port(9000).tls(true).build()`:
   - Fluent Builder pattern. Chainable `mut self` consumption avoids long parameter lists and prevents boolean blindness.

8. `if discriminant(&ev1) == discriminant(&ev2)`:
   - `std::mem::discriminant()` comparison. Compares internal enum discriminant tags without inspecting or cloning the payload data.

---

### 🌟 Adhyaya 3 Concluded — Agle Kadam
Is chapter me humne Structs, Enums as ADTs, Zero-Sized Types, Memory Layouts, Recursive Types, Typestates, NPO, aur Pattern Matching ko complete systems depth me conquer kar liya hai.  
Agla Adhyaya (**Volume 04**) Collections, Data Structures (`Vec`, `HashMap`, `BTreeMap`), UTF-8 Strings (`String` vs `&str`), aur Functional Iterator Engine par dedicated hai:  
👉 **[Volume 04: Collections, Data Structures & The Iterator Engine](file:///c:/Dev/Rust/rust_book/04_collections_and_data_structures.md)**
