fn main() {
    println!("Hello, world!");
    test_printdata(5);

    let x = 5;
    
    let y = {
        let x = 3;
        x + 1
    };

    println!("Test Rust Fun {}" , y);
    
    let test_condition = 5;
    let test = if test_condition == 5 {
        6
    } else {
        7
    };

    println!("test_condition {}", test);

    let list = [1,2,3,4,5,6];

    for value in list.iter() {
        println!("for loop value {}", value);
    }

    for value in 1..3 {
        println!("for loop value 2 {}", value)
    }

    let s1 = String::from("TEST");
    let s2 = s1;
    println!("S2 {}", s2);

    let s1 = String::from("TEST1");
    let s2 = s1.clone();

    println!("TEST3 {} , {}", s1, s2);

    println!("OWNER");
    
    let test1 = create_string();
    let test2 = move_owner(test1);

    println!("TEST END {}", test2);
    
}

fn test_printdata(test: i32) {
    println!("TEST Print Data {}", test)
}

fn create_string() -> String {
    String::from("TEST1")
}

fn move_owner(value: String) -> String {
    value
}