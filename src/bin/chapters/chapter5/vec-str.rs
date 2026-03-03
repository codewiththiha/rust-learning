fn main() {
    let v1 = "Hello";
    println!("{}", v1);
    let mut v2 = String::from("New String");
    v2.push_str("Is here");
    println!("{}", v2);

    // replace creates a new copy because it's safer and easier for the computer to build a new string
    // from scratch than to try and "stretch" an existing string in the middle.
    let replaced = v2.replace("String", "Rust");
    for w in replaced.split_whitespace() {
        println!("{}", w);
    }

    println!("Replaced:{}", replaced);
    let b1 = replaced.as_bytes();
    println!("{}", b1.len());
    let c_to_string = v1.to_string();
    let slice_demo = &c_to_string[0..4];
    println!("{}", slice_demo);
    let char_demo = c_to_string.chars().next().unwrap();
    println!("{}", char_demo);
    if replaced.contains("Rust") {
        println!("Contains rust!!");
    }
}
