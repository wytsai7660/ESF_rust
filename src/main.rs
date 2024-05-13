fn main() {
    let reference;
    {
        let str = String::from("Hello, World!");
        reference = &str;
    }
    print_but_dont_take(reference);

    // let str = String::from("Hello, World!");
    // take_and_return_ref(str);
}

#[allow(dead_code)]
fn print_but_dont_take(str: &String) {
    println!("{}", str);
}

// fn take_and_return_ref(str: String) -> &String {
//     &str
// }
