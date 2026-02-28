fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y;
}

fn average_of_three(x: i32, y: i32, z: i32) {
    println!("{}", (x + y + z) / 3);
}

fn main() {
    let x = sum(1, 2);
    let y = multiply(2, 4);
    average_of_three(1, 2, 4);
    println!("sum result {}, multiply result {}", x, y);
}
