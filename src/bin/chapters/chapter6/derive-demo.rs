use std::fmt;

// this is a powerful macro that compile on its own before actual main code get compile
// so it's like having own language inside the language , Cool!!!!!!

//// mental-model
// The Debug License: #[derive(Debug)]
// What it unlocks: The {:?} formatter.
// Why you need it: Developers need it for logging. If your game crashes, you want to see exactly which variant of the enum caused it, including its internal data.
//
// Without it: println!("{:?}", my_error) will cause a compiler error.
//
// With it: The compiler writes a hidden function that "inspects" your enum and prints it like this: FileNotFound("missing.txt").
#[derive(Debug)]
enum GameError {
    FileNotFound(String),
    InvalidInput(String),
    NetworkError(String),
    SnakeCollision,
}

// read ./notes/mental-model.md if you want to understand how formatter works
// this simple impl example is also very strong example of how powerful rust is
// this syntax mean : our GameError enum is trying to become member of Display
// and implement required methods and now it becomes a member and can use in
// places where Display can also use
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::FileNotFound(path) => write!(f, "File Not Found {}", path),
            GameError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            GameError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            GameError::SnakeCollision => write!(f, "Snake collided!"),
        }
    }
}

fn load_config(path: &str) -> Result<String, GameError> {
    let collision = false;
    if path.is_empty() {
        return Err(GameError::InvalidInput("Path is empty".to_string()));
    }

    // Simulate file loading
    if path == "missing.txt" {
        return Err(GameError::FileNotFound(path.to_string()));
    }

    if path == "404" {
        return Err(GameError::NetworkError("404".to_string()));
    }

    if collision {
        return Err(GameError::SnakeCollision);
    }

    Ok("config data".to_string())
}

fn new_func() -> Result<String, GameError> {
    let config = load_config("missing.txt")?; //  Won't compile!
    Ok(config)
}

fn main() {
    match load_config("missing.txt") {
        Ok(config) => println!("Config: {}", config),
        Err(e) => println!("Error: {:?}", e),
    }
    let config = new_func().unwrap_or(String::from("Err"));
    println!("{}", config);
}
