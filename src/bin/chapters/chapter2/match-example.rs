fn main() {
    let number = 10;

    match number {
        1..=5 => println!("Small: {}", number),
        6..=10 => println!("Medium: {}", number),
        _ => println!("Large: {}", number), // _ this isn't ignore in rust , it acts like a default
    }
}
