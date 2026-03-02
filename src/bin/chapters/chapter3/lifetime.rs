// fn get_reference() -> &String {
//     let s = String::from("Hello");
//     &s // this act like it get converted to a stack even tho it's a heap
// } // and s dropped here
//
fn get_string() -> String {
    let s = String::from("World");
    s // Return owned string
}

// lifetime example
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let mut s2 = get_string();
    println!("s2:{}", s2);
    s2 = String::from("HH");
    println!("s2:{}", s2);
    println!("get_string {}", get_string());

    // Compiler's Logic: 1. string1 lives for 6 lines. string2 lives for 3 lines.
    // 2. The "Shortest Common" lifetime is 3 lines.
    // 3. Therefore, 'a = 3 lines.
    // 4. Since result is used within those 3 lines, it is Safe.
    let string1 = String::from("long"); // Scope A starts
    {
        let string2 = String::from("xyz"); // Scope B starts
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest is {}", result);
    } // Scope B ends (string2 is dropped)
}
