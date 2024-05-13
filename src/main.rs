fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let n: i32 = 9;
    // let n = 10;
    let mut m = add(n, -5);

    println!("immutable variable n = {}", n);
    println!("mutable variable m = {m}");

    for i in 0..n {
        print!("{i} ");
        // println!("Hi");
    }
    println!();

    if n > m {
        println!("n is greater than m");
    } else if n == m {
        println!("n is equal to m")
    } else {
        println!("n is less than m");
    }

    while m > 0 {
        println!("m = {}", m);
        m -= 1;
    }

    match n % 5 {
        0 => println!("n is multiple of 5"),
        remainder => println!("n is not multiple of 5, remainder is {remainder}"),
    }

    if let 10 = n {
        println!("n is 10");
    }
}
