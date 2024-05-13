fn main() {
    let str = String::from("Hello, World!");

    let ref1 = &mut str;
    let ref2 = &mut str;

    modify_but_dont_take(ref1);
    modify_but_dont_take(ref2);

    // println!("{}", str);

    // print_but_dont_take(ref1);
    // print_but_dont_take(ref2);
}

#[allow(dead_code)]
fn print_but_dont_take(str: &String) {
    println!("{}", str);
}

#[allow(dead_code)]
fn modify_but_dont_take(str: &mut String) {
    str.push_str("!");
}
