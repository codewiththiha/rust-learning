fn main() {
    // Rule:1 Each value has exactly ONE owner

    // Rule:2 When the owner goes out of scope, the value is dropped
    {
        let s = String::from("Hello");
        println!("{}", s); // ok!
    }
    // println!("{}", s); // !ok :))

    // Rule:3 There can only be one mutable reference OR multiple immutable references (not both)
    let s = String::from("Hello");
    let b1 = &s;
    let b2 = &s;

    // we can't since we already have immutable references
    // let b3 = &mut s;
    println!("{}, {}", b1, b2);

    // {
    //     // WRONG: Can't have mutable and immutable refs together
    //     let mut s = String::from("hello");
    //     let r1 = &s;
    //     let r2 = &mut s; // ERROR!
    //
    //     // CORRECT: Immutable refs only
    //     let r1 = &s;
    //     let r2 = &s; // OK - multiple immutable refs allowed
    //
    //     // CORRECT: One mutable ref only
    //     let r1 = &mut s;
    //     // let r2 = &mut s;  // ERROR - only one mutable ref
    //     println!("r {}", r1);
    // }

    // using immutable and mutable without breaking rule example:
    let mut m1 = 43;
    {
        let m1_r = &m1;
        println!("value or m1_r: {}", m1_r);
    } // m1_r reference's lifetime ends here
    let m2_r = &mut m1;
    *m2_r = 70;
    //  The Rule: If you have an active Mutable Reference (&mut), the original owner is "locked."
    //  You cannot use the name m1 to read or write until the mutable reference m2_r is finished.
    //  println!("value of m1 and m2_r: {} , {}", m1, m2_r);
    println!("value of m1 and m2_r: {}", m2_r);
    println!("value of m1 and m2_r: {}", m1);

    // in case confused
    {
        let mut m1 = 43;
        let m2_r = &mut m1; // Lock Starts

        *m2_r = 70; // Writing through the lock
        println!("{}", m2_r); // OK: This is the LAST use of m2_r. 
        // 🔑 Lock is released RIGHT HERE.

        println!("{}", m1); // OK: The lock is gone, m1 is free to be read!
    }
}
