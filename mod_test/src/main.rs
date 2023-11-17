extern crate mod_test;

fn main(){
    println!("Hello mod_test");

    mod_test::network::connect();
    mod_test::client::mod1_client::connect();
}
