enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn print_address(&self) -> () {
        match &self {
            IpAddr::V4(x1,x2,x3,x4) => println!("IPV4: {}.{}.{}.{}",x1,x2,x3,x4),
            IpAddr::V6(str) => println!("IPV6: {}",str)
        };
    }
}

fn main() {
    let home = IpAddr::V4(128,255,255,1);
    let loopback = IpAddr::V6(String::from("::1"));

    home.print_address();
    loopback.print_address();

    //We have an option with two values
    // Some(val)
    // None
    let some_number = Some(5);
    let some_string = Some("string");
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);

    //it's useful with match operator
    let x = Some(42 as u8);
    match x {
        Some(val) => println!("Value: {}", val),
        None      => println!("No value found"),
    }

    //there is also if let construction
    let some_value = Some(8);

    if let Some(8) = some_value {
        println!("::::)");
    }

    //also there is while let
    let so_eighty = [8, 8, 8, 88, 8];
    let mut iter8or = so_eighty.iter();

    while let Some(8) = iter8or.next() {
        println!("∞");
    }

    let numbers = [1, 2, 3].iter();                   // std::slice::Iter
    let chars   = "abc".chars();                      // std::str::Chars
    let words   = "one two three".split_whitespace(); // std::str::SplitWhitespace

    println!("{:?}", numbers);
    println!("{:?}", chars);
    println!("{:?}", words);

    let numbers: Vec<&u32> = [1, 2, 3].iter().collect();
    println!("{:?}", numbers);

    let string = String::from("Petko");
    let mut chars = string.chars(); // Mutable!

    while let Some(c) = chars.next() {
        println!("{:?}", c);
    }

    for c in chars {
        println!("{:?}", c);
    }

    //guard to match
    let pair = (2, -2);

    match pair {
        (x, y) if x == y                   => println!("Едно и също"),
        (x, y) if x + y == 0               => println!("Противоположни"),
        (x, y) if x % 2 == 1 && y % 2 == 0 => println!("X е нечетно, Y е четно"),
        (x, _) if x % 2 == 1               => println!("X е нечетно"),
        _                                  => println!("Нищо интересно"),
    }

    let age: i32 = -5;

    match age {
        n if n < 0 => println!("Ще се родя след {} години.", n.abs()),
        0          => println!("Новородено съм."),
        1 ..= 12   => println!("Аз съм лапе."),
        13 ..= 19  => println!("Аз съм тийн."),
        _          => println!("Аз съм дърт."),
    }

    //patterns
    let (a,b) = (1,69);
    println!("{}",b);

    if let Some(val) = Some(5) { // -> Refutable pattern
        println!("Okay!");
    }
}
