fn main() {
    let str = String::from("Hello, World!");

    let str2 = take_and_return(str);
    // println!("{}", str); // not owner anymore
    println!("{}", str2);

    // print_but_dont_take(&str);
    // println!("{}", str);

    // modify_but_dont_take(&mut str);
    // println!("{}", str);
}

#[allow(dead_code)]
fn take_and_return(str: String) -> String {
    str
}

#[allow(dead_code)]
fn print_but_dont_take(str: &String) {
    println!("{}", str);
}

#[allow(dead_code)]
fn modify_but_dont_take(str: &mut String) {
    str.push_str("!");
}
