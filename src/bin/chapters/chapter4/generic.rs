// // This is how we can do if Option was defined by us not built in the detail note in notes
// enum CustomOption<T> {
//     Custom(T),
//     Null,
// }
//
// impl<T: std::fmt::Display> CustomOption<T> {
//     fn custom_method(&self) {
//         match self {
//             CustomOption::Custom(val) => println!("Get, {}", val),
//             CustomOption::Null => println!("Get nothing"),
//         }
//     }
// }

// 1. Define the "behavior"
trait Printable {
    fn custom_method(&self);
}

// 2. Implement that behavior for the foreign type Option<T>
// Note the <T> after 'impl'—this "declares" T for this block.
impl<T: std::fmt::Display> Printable for Option<T> {
    fn custom_method(&self) {
        match self {
            Some(val) => println!("Get, {}", val),
            None => println!("Get nothing"),
        }
    }
}

fn main() {
    // Some variants
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("hello");

    // None variant
    let absent: Option<i32> = None;

    some_number.custom_method();
    some_string.custom_method();
    absent.custom_method();

    // Can't use Option<T> as T directly
    // let x: i32 = some_number;  // ERROR!

    // Must handle both cases with a match
}
