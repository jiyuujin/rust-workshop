fn main() {
    println!("Hello, world!");

    // 変数宣言 (束縛) はイミュータブル、再代入不可
    let i = 1;
    println!("{}", i);

    // 再代入させる場合は mut を使う
    let mut j = 1;
    j = 2;
    println!("{}", j);

    let num1 = 10;
    // if 式
    let fizzbuzz1 = if num1 % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num1 % 3 == 0 {
        "Fizz".to_string()
    } else if num1 % 5 == 0 {
        "Buzz".to_string()
    } else {
        num1.to_string()
    };
    println!("{}", fizzbuzz1);

    // Iterator トレイト
    for num2 in 0..100 {
        let fizzbuzz2 = if num2 % 15 == 0 {
            "FizzBuzz".to_string()
        } else if num2 % 3 == 0 {
            "Fizz".to_string()
        } else if num2 % 5 == 0 {
            "Buzz".to_string()
        } else {
            num2.to_string()
        };
        println!("{}", fizzbuzz2);
    }
}
