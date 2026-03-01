fn main() {
    // C's equivalent to malloc(sizeof(string))
    let s = String::from("Hello");
    println!("{}", s);

    // get out of scope example
    {
        let s2 = String::from("New Hello");
        println!("{}", s2);
    } // it out of scope once it passed "}" closing brace
    // we can't do that the since s2 will get out of scope
    // just like how C's stack var can't survuve outside functions
    // in rust we are getting that behaviour on heap vars
    // println!("{}", s2);

    // owner passing (move) example
    let o1 = String::from("Owner");
    let o2 = o1;

    // this won't be possible since our o1 is no longer owner but o2 is
    //   println!("{}", o1);
    println!("{}", o2);

    // owner copy example
    // move ownershipp example
    {
        let o1 = String::from("Owner");
        let o2 = o1.clone();
        println!("{}", o1);
        println!("{}", o2);
        // both worked , so note : heap default behaviour is moving
    }

    // other types' default is copy not move like in heap
    {
        let x = 32;
        let y = x;
        println!("{}", x);
        println!("{}", y);
        // since they copy both works
    }
}
