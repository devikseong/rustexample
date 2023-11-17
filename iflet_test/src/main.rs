fn main() {
    println!("Hello, world!");

    let test_let: Option<u8> = Some(5);
    let result = if let Some(5) = test_let {
        println!("OK True");
        3
    } else {
        println!("No False");
        4
    };

    println!("Result Value {}", result);
}
