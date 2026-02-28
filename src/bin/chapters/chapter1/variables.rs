fn main() -> () {
    let x: u8 = 5;
    println!("{}", x);
    //   x = 6; // This will not work (in rust it's immutable by default)

    let mut y: u16 = 7;
    println!("Before {}", y);

    y = 9;
    println!("After {}", y);

    // shadowing example
    // better type conversions :))
    let x = "Something";
    let x = x.len();
    let x = x + 1;
    println!("The value of x {}", x);

    // safe reference example
    let data: Vec<i32> = vec![1, 2, 3];
    let first: &i32 = &data[0]; // Reference (Reader)

    // if we push to a vec it'll assign with new address which will make the pointer aka reference
    // point to the old (dead) address which will cause crush to the program and rust compiler yell
    // at us :)) so cute rust!

    // data.push(4);
    println!("{}", first);
}
