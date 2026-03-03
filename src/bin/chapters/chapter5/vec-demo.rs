trait Print {
    fn print_vec(&self);
}

impl<T: std::fmt::Display> Print for Vec<T> {
    fn print_vec(&self) {
        for i in self {
            print!("{},", i);
        }
        println!("");
    }
}

fn modify_vec(v: &mut Vec<i32>) {
    v.push(99);
}

fn main() {
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    modify_vec(&mut v1);
    // if we gave for loop the v1 with no & symbol we gave away full ownership and in the end
    // compiler will destory the v1 completely
    // 2 ways to get value
    let first = &v1[0];

    // this will be equlivalent to using [1] instead
    // :)) since we used .unwrap
    let second = v1.get(1).unwrap();
    println!("Capacity, {}", v1.capacity());
    println!("{},{}", first, second);
    println!("Before pop");
    for n in &v1 {
        println!("{}", n);
    }
    println!("After pop");
    v1.pop();
    for n in &v1 {
        println!("{}", n);
    }
    // or use .remove() to remove specific index (pop only removes last index)
    v1.remove(1);
    for (ind, number) in v1.iter().enumerate() {
        println!("{},{}", ind, number);
    }
    vec_methods();
}

fn vec_methods() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.clear();
    // next() return an option enum back
    println!(
        "{}",
        match v.iter().next() {
            Some(_s) => "Something",
            None => "Nothing",
        }
    );
    v.extend(vec![8, 9, 10, 11, 12, 14]);

    // Find index
    // unlike next this can go to specific index
    // We use `if let` when we only care about one case (the `Some` case) and want to ignore the other one (`None`).
    // so .position also return an Option
    if let Some(idx) = v.iter().position(|x| *x == 3) {
        println!("Found at index {}", idx);
    }

    v.print_vec();
    v.truncate(3);
    v.print_vec();
    // or we can borrow and see without modifying actual vec
    // here is example for that
    let slice_v = &v[1..2];
    println!("Here is slice_v");
    for i in slice_v {
        println!("{}", i);
    }
}
