enum CoinDetail {
    CotinDetail1,
    CotinDetail2
}

enum Coin {
    TestCoin1,
    TestCoin2(CoinDetail)
}

fn match_test(coin:Coin) -> u32 {
    match coin {
        Coin::TestCoin1 => 8,
        Coin::TestCoin2(value) => {
            5
        }
    }
}

fn option_check(value: Option<i32>) -> Option<i32> {
    match value {
        Some(i) => Some(i + 1),
        None => None
    }
}

fn main() {
    println!("Hello, world!");
    let value = match_test(Coin::TestCoin1);
    let value2 = match_test(Coin::TestCoin2(CoinDetail::CotinDetail1));
    println!("main Test {} {}", value, value2);

    let value_test = option_check(Some(5));
    let value_none = option_check(None);
    println!("main Test 2 {:?} {:?}", value_test, value_none);
}
