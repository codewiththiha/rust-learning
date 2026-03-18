use std::{
    cell::RefCell,
    ops::Deref,
    rc::{self, Rc},
};

pub fn refcell() {
    let r = RefCell::from(String::from("Hello World"));
    *r.borrow_mut() = "AYO".to_string();
    println!("{}", &*r.borrow());

    let i = RefCell::from(12);
    let mut i2 = i.borrow_mut();
    *i2 = 0;
    println!("{}", *i2);
}

pub fn rc_demo() {
    let x = Rc::from(1);
    let y = Rc::clone(&x);
    let z = Rc::clone(&x);
    //// This won't possible
    //z = 5.into();

    println!("{}", y)
}

pub fn demo_both() {
    let x = Rc::new(RefCell::from(3));

    // this rc clone only copy the address
    // and rc also keep track of the owners' count
    let y = Rc::clone(&x);
    let z = Rc::clone(&x);
    *y.borrow_mut() = 79;

    // expected 79
    println!("{}", &*z.borrow());
}

pub fn main() {
    refcell();
    rc_demo();
    demo_both();
}
