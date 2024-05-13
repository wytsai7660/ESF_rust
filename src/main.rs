fn main() {
    // let ref: &mut String;
    let reference;
    {
        let str = String::from("Hello, World!");
        reference = &str;
    }
    print_but_dont_take(reference);
}

#[allow(dead_code)]
fn print_but_dont_take(str: &String) {
    println!("{}", str);
}
