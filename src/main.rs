fn main() {
    let mut str = String::from("Hello, World!");
    // look_but_dont_take(&str);

    // let str2 = take_and_return(str);
    append_but_dont_take(&mut str);

    println!("{}", str);

    // println!("{}", str); // not owner anymore
    // println!("{}", str2);
}

// fn take_and_return(str: String) -> String {
//     str
// }

// fn look_but_dont_take(str: &String) {
//     println!("{}", str);
// }

fn append_but_dont_take(str: &mut String) {
    str.push_str("!");
}
