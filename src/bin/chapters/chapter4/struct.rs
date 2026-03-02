struct Point {
    x: i32,
    y: i32,
}
struct Color(i32, i32, i32); // unnamed struct example

struct User {
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let p1: Point = Point { x: 12, y: 20 };
    println!("x,y {},{}", p1.x, p1.y);

    let mut p2 = Point { x: 5, y: 5 };
    p2.x = 10;
    println!("x,y {},{}", p2.x, p2.y);

    let mut user1 = User {
        name: String::from("John"),
        email: String::from("john@mail.com"),
        active: true,
    };
    user1.name = String::from("John Doe");
    println!("{},{},{}", user1.name, user1.email, user1.active);

    let user2 = User {
        name: String::from("World"),
        ..user1 // well this is neat!
    };
    println!("{},{},{}", user2.name, user2.email, user2.active);

    let black = Color(0, 0, 0);
    let d_black = destructure_color(black);
    println!("{}", d_black.0);
}

fn destructure_color(c: Color) -> Color {
    // just for demo void "()" should be more make sense
    let Color(r, g, b) = c; // destructure pattern
    println!("{},{},{}", r, g, b);
    return Color(r, g, b);
}
