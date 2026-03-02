fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Can't divide by zero"));
    } else {
        return Ok(a / b);
    }
}

fn main() {
    let result = divide(4, 7);
    match result {
        Result::Ok(v) => println!("result{}", v),
        Result::Err(e) => println!("{}", e),
    }
}

// {
//     fn read_file(path: &str) -> Result<String, std::io::Error> {
//     // ? propagates errors automatically
//     let mut file = std::fs::File::open(path)?; // pretty neat!
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }
//
// // Equivalent without ?:
// fn read_file_verbose(path: &str) -> Result<String, std::io::Error> {
//     let mut file = match std::fs::File::open(path) {
//         Ok(f) => f,
//         Err(e) => return Err(e),
//     };
//     // ... more code
// }
// }
