fn main() {
    println!("Hello Test3");

    let test: i32 = 100000;
    println!("Heelo print value {}", test);

    let x: (i32, char, f64, u8) = (500, 'C', 50.4, 1);
    let test1 = x.0;
    let test2 = x.1;
    let test3 = x.2;
    let test4 = x.3;

    println!("test1 {}, test2 {}, test3{}, test4{}", test1, test2, test3, test4);
}
