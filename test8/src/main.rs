
#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Test1,
    Test2(i32),
    Message(String),
    Value(i32,i32,i32)
}

impl Message {
    fn call(&self) -> &str {
        "TEST1 Message"
    }
}

fn main() {
    println!("Hello, world!");

    let addr_v6 = IPAddr::V6(String::from("TEST V6"));
    println!("Addr Value {:?}", addr_v6);

    let message = Message::Message(String::from("TEST1"));
    println!("Addr Value1 {:?}", message);
    println!("Addr Value2 {:?}", message.call());
}
