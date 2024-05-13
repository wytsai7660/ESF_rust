fn add(a: i32, b: i32) -> i32 {
    return a + b;
    // a + b
}

fn main() {
    let n = 10;

    let m = n > 5 ? 999 : 111;

    // let m;
    // if n > 5 {
    //     m = 999;
    // } else {
    //     m = 111;
    // }

    // let m = if n > 5 { 999 } else { 111 };

    // let m = match n {
    //     0 => 1,
    //     1 => 2,
    //     2 => 3,
    //     3 => 4,
    //     _ => -1,
    // };

    println!("n = {}", n);
    println!("m = {}", m);

    add(n, m);
}
