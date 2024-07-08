use std::io;


#[allow(unused)]
fn main() {
    let x = 5;
    // Shadowing from now on the compiler sees the value of x as 6
    let x = x + 1;

    {
        // Shadowing only in this scope, value of x is in this scope is 12 and out of it is again 6
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    let mut tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value x i {x}");

    println!("The value of x is: {x}");

    let nums = [1,2,3,4,5];

    let mut idx = String::new();

    io::stdin().read_line(&mut idx).expect("Something went wrong");

    let idx: usize = idx.trim().parse().expect("must be a number");

    let ele = nums[idx];

    println!("{ele}");

}