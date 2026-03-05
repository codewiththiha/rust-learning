#[derive(PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn main() {
    let d1 = Direction::Up;
    let d2 = Direction::Down;

    // without derive PartialEq this won't compile
    if d1 == d2 {
        println!("Same direction");
    }

    let d_vec = vec![Direction::Left, Direction::Right];

    // notice it demand the & because it don't want to waste
    // cpu on copying
    if d_vec.contains(&Direction::Right) {
        println!("Include right direction");
    }
}
