// there're three loops in rust
// loop (infinite loop) equivalent to while(1)
// while loop
// for loop
// all loops act as expression meaning we can assign to vars
// and label loops too (might use rarely)
// the same goes to vars
fn main() {
    // loop example
    let mut i = 0;
    loop {
        // i just notice rust doesn't support ++ or -- operators
        // i ++;
        i += 1;
        if i == 10 {
            break;
        };
    }
    println!("i after loop: {}", i);

    // while loop example
    let mut x = true;
    while x {
        i += 1;
        if i == 20 {
            x = false;
        }
    }
    println!("i after while: {}", i);

    // for loop example
    for i in 0..=5 {
        println!("Iterating i in for loop :{i}");
    }

    // reverse for loop example
    for i in (0..=5).rev() {
        println!("Iterating i in reverse for loop :{i}");
    }
    // this was what i first thought but this won't work
    // // mutating array example
    // let mut arr = [1, 2, 3];
    // let mut owner_arr = &arr;
    // for i in owner_arr {
    //     owner_arr[i] *= 2;
    // }

    // correct way to do

    // mutating array example
    let mut arr = [1, 2, 3];
    let owner_arr = &mut arr;
    for i in 0..3 {
        owner_arr[i] *= 2;
    }

    // and more simpler way
    for i in &mut arr {
        *i *= 2;
    }

    // loop return a value
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 20 {
            break count * 2; // this act as final return value
        }
    };
    println!("result :{}", result);

    // label loop example
    'outer: for i in 0..5 {
        println!("i:{}", i);
        for j in 0..5 {
            println!("j:{}", j);
            if i == 2 && j == 2 {
                break 'outer; // breaking outer loop
            }
        }
    }
}
