// 1. THE PLACEHOLDER
println!("Error: {}", e)
//            ^^
//            This is just a hole in the text. 
//            It tells Rust: "Put 'e' here."

// 2. THE TRAIT LOOKUP
// Rust looks at 'e' (GameError).
// It asks: "Does GameError implement Display?"
// ✅ Yes → It calls e.fmt(&mut formatter)

// 3. THE FORMATTER (Buffer)
// Rust creates a temporary buffer (the Formatter 'f').
// It passes 'f' into your fmt() function.

// 4. THE WRITE! MACRO (Inside your fmt)
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write! puts text INTO the buffer 'f'.
        // It does NOT return a String.
        write!(f, "Snake collided!") 
    }
}

// 5. THE PRINT
// Once fmt() is done, println! takes the buffer 'f'
// and sends the final text to the Console (stdout).
