// am just testing void return you don't need to explictly does this
fn main() -> () {
    // there's no null in rust that's why you can't
    // directly convert from f64 to pointer i64
    // first we need to understand the type we first get
    // from &x in ptr var is a reference not a

    let x = 3.14;
    let ptr = &x as *const f64;
    let int_ptr = ptr as *const i64;

    unsafe {
        println!("Garbage print {}", *int_ptr);
    }
}
