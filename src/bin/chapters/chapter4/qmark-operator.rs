use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
//// Without we have to write like this
// fn read_file_no_qm(path: &str) -> Result<String, std::io::Error> {
//     let mut file = match std::fs::File::open(path) {
//         Ok(f) => f,
//         Err(e) => return Err(e),
//     };
//     let mut contents = String::new();
//     match file.read_to_string(&mut contents) {
//         Ok(r) => r,
//         Err(e) => return Err(e),
//     };
//     Ok(contents)
// }

// We need interfaces to add more methods for defined (built in) Enums
// for is like asking it to follow the trait (interface)'s rules
trait PrintIf {
    fn print_method(&self);
}

// I first tried with single T with with the same constraint but the Result's type isn't just check
// the type it assume Result<T,T> as it should same in both for eg if Ok is i32 Err should i32
// so my logic failed my logic was i'm asking compiler to work if they follow Display's rule
impl<T: std::fmt::Display, E: std::fmt::Display> PrintIf for Result<T, E> {
    fn print_method(&self) {
        match self {
            Ok(v) => println!("Value {}", v),
            Err(e) => println!("Err {}", e),
        }
    }
}

fn main() {
    let result = read_file("");
    match result {
        Result::Ok(s) => println!("{}", s),
        Result::Err(e) => println!("{}", e),
    }
    // example
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("error");

    // is_ok / is_err
    println!("{}", ok.is_ok()); // true
    println!("{}", err.is_err()); // true

    // unwrap (panic if Err)
    let val = ok.unwrap(); //     // let val2 = err.unwrap();

    // unwrap_or (default if Err)
    let val3 = err.unwrap_or(0); //     
    // map (transform Ok value)
    let doubled = ok.map(|n| n * 2); // Ok(10)
    // map_err (transform error)
    let with_msg = err.map_err(|e| format!("Error: {}", e));
    with_msg.print_method();
    doubled.print_method();
    println!("{} {}", val, val3);
}
