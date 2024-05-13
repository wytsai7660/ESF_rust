fn main() {
    let mut str = String::from("Hello, World!");

    let mut ref1 = &mut str;
    let mut ref2 = &mut str;

    modify_but_dont_take(ref1);
    modify_but_dont_take(ref2);
    println!("{}", str);
}

#[allow(dead_code)]
fn modify_but_dont_take(str: &mut String) {
    str.push_str("!");
}
