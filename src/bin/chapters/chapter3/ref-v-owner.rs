// fn own(get: String) {
//     println!("{}", get); // value dropped here
// }

fn reference(get: &String) {
    println!("{}", get);
}

fn main() {
    let org = String::from("Org");
    // own(org);
    // println!("{}", org); // this will cause error since the value is dropped
    reference(&org);
    println!("{}", org); // still possible
}
