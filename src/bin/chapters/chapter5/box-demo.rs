// If we do this we won't be able to deny and put None
// We have to put next no matter what , which is what we don't want
// struct Segment {
//     value: i32,
//     next: Box<Segment>,
// }

// this is also a choice but if we do that we have to touch unsafe blocks
// so rust compiler simply doesn't enjoy , dealing with raw pointers
// struct Segment {
//     value: i32,
//     next: *mut Segment,
// }

// so the prefer way is
struct Segment {
    value: i32,
    next: Option<Box<Segment>>,
}

fn main() {
    let segment = Segment {
        value: 1,
        next: Some(Box::new(Segment {
            value: 2,
            next: None,
        })),
    };
    println!("{},{}", segment.value, segment.next.unwrap().value);
}
