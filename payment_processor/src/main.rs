// The intergce (the Contract)
// Any struct that wants to act as a payment backend Must implement this trait
// This defines the interface. Notice that the method charge_card has no body. It just ends in a semicolon. A trait says what must be done not how to do it 
// &self - Both implementations require an immutable reference to the struct Stripe needs its API key, MockBackend needs its boolean flag

// It defines an interface the job description.
// The trait keyword tells the Rust compiler we are defining shared behavior not data. We name it PaymentBackend (pascalCase)

trait PaymentBackend {

// We define the function signature. Notice there are no curly braces and no code body. It just ends with semicolon . A trait says what must exist not how it works
// &self - This means when this function is called it gets an immutable reference to the struct calling it We don't use &mut self because reading an API key doesn't require mutating the struct 
// Result<(), String> The function must returns a Result enum. The Ok variant must contains () the empty unit type , meaning nothing to return , just success. The Err variant must contains a String the error message 
    fn charge_card(&self, amount: f64) -> Result<(), String>;
}

// The production backend
// Defines two completely different data structures in memory 
// The struct keyword creates custom data types
// Why do we do ut - Stripe needs an API key to talk to the internet. mockBackend doesn't use the internet it just needs a boolean switch so our automated tests can force it to succeed or fail on demand
struct Stripe {
    api_key: String,
}


// This is rust version of inheritance/interfaces. We are officially saying Stripe promises to have a charge_card method with the exact signature defined in the trait
// We force stripe to sign the contract
// It officially gives the Stripe struct the Cashier nametag
// This syntax is Rust way of attaching an interface to a type 
impl PaymentBackend for Stripe {
    fn charge_card(&self, amount: f64) -> Result<(), String> {

        // Imagine this makes a real HTTP request to Stripes servers
        // Because we took &self as the first argument, we can access the data inside the Stripe struct using dot-notation
        // This proves to the compiler that Stripe fulfills the PaymentBackend contract. We wrote teh exact same logic for Mock Backendss
        println!("[STRIPE] charging ${:.2} using API key : {}", amount, self.api_key);

        Ok(())
    }
}

// Fake testing backed
struct MockBackend {
    should_succeed: bool,
}

// We force MockBackend to sign the exact same contract

// the impl for syntax is Rust way of attaching an interface to a type .
// self.api_key - Because we took &self as the first argument, we can access the data inside the Stripe struct using dot-notation

// This proves to the compiler that Stripe fulfills the PaymentBackend contract. We wrote the exact same logic for Mockbackend
impl PaymentBackend for MockBackend {
    fn charge_card(&self, amount: f64) -> Result<(), String> {
        // No HTTP request here . Just fake logic for automated tests
        if self.should_succeed {
            println!("[MOCK] successfully faked a charged for ${:.2}", amount);
            Ok(())
        } else {
            println!("[MOCK] Faked a card decline for ${:.2}", amount);
            Err("Card declined by mock".to_string())
        }
    }
}


fn main() {
    // We will build the PaymentProcessor in the next step
    println!("Backends are ready!");
}