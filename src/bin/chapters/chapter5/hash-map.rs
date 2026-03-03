use std::collections::HashMap;

fn main() {
    let mut new_hsh = HashMap::new();
    new_hsh.insert("Alice", 19);
    new_hsh.insert("John", 20);
    new_hsh.remove("John");
}
