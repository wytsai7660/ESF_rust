// use std::io;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// fn readInt<T: std::str::FromStr>() -> T
// where
//     T::Err: std::fmt::Debug,
// {
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     guess.trim().parse().expect("Please type a number!")
// }

fn main() {
    // Let's do some stupid things

    let m = (let n = 10);
    // let n = 10;

    // n = 20;

    // println!("{} {}", n);

    // if (n < 0) {
    //     println!("n is negative");
    // }

    // match n {
    //     0 => println!("zero"),
    //     1 => println!("one"),
    //     other => println!("{}", other),
    //     _ => println!("other"),
    // }

    // let input = readInt::<i32>();
    // println!("{}", i32::MAX + input);

    // let input = readInt::<usize>();
    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{}", arr[input]);
}
