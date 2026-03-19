fn clone_demo() {
    let eg: &Option<&String> = &Some(&String::from("Hello"));
    // clone will dereference 1 level from &Some to Some but won't touch the inner value
    let eg2: Option<&String> = eg.clone();
    // cloned clone the inner wrapped value
    let eg3: Option<String> = eg.cloned();
    // this's combined usage example
    let eg4: String = eg.cloned().unwrap();
    println!("{}", eg3.unwrap());
}

fn main() {
    clone_demo();
}
