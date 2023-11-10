fn main() {
    let s = String::from("Hello");
    let size = test_function(&s);

    println!("Hello, world! data {}, size {}", s , size);

    let mut s1 = String::from("Hello1");
    test_function1(&mut s1);

    println!("Test Borrow {} ", s1);

    println!("TEST dangle reference {}", test1_fun());

    let mut slice_test: String = String::from("TEST");
    let len = test1_slice1(&slice_test);

    println!("TEST SLICE1 {}", slice_test);
    slice_test.clear();
    println!("TEST SLICE2 {} , fail [{}]", len, slice_test);

    //String Literal Slice
    let a = String::from("TEST1 TEST2");
    let a1 = &a[0..4];
    let a2 = &a[..4];
    let a3 = &a[4..];
    let a4 = &a[..];

    println!("TEST a {}", a);
    println!("TEST a1{}", a1);
    println!("TEST a2 {}", a2);
    println!("TEST a3 {}", a3);
    println!("TEST a4 {}", a4);

    let mut slice_test2 = String::from("TEST1 TEST2");
    let slice_result2 = test1_slice2(&slice_test2);
    slice_test2.clear();

    println!("slice_reult2 {}", slice_result2);
}

fn test_function(s1: &String) -> usize {
    s1.len()
}

fn test_function1(s1: &mut String) {
    s1.push_str(" TEST");
}

fn test1_fun() -> String {
    let s1 = String::from("test1");

    s1
}

fn test1_slice1(s1: &String) -> usize {
    let bytes = s1.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    
    s1.len()
}

fn test1_slice2(s1: &str) -> &str {
    let bytes = s1.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s1[0..i];
        }
    }
    
    &s1[..]
}