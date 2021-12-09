fn set_dimentions((width, height): (u32, u32)) {
    todo!()
}

/// Push word "Kamenov" to a given string
/// Basic usage:
/// ```
/// let mut s = String::from("foo");
/// change_string(& mut s);
/// assert_eq!("foo Kamenov",s);
/// ```
fn change_string(s : & mut String) -> (){
    s.push_str(" Kamenov");
}

fn panic() {
    let opt : Option<isize> = None;
    opt.unwrap();
}

#[test]
#[should_panic]
#[cfg(test)] //to compile only when test are executed
fn cant_connect_to_invalid_ip() {
    panic();
}

fn main() {
    let mut s = String::from("Petko");
    change_string(& mut s);
    println!("{}",s);

    //String work
    let bytes: Vec<u8> = "Здравей! 😊".bytes().collect();
    let chars: Vec<char> = "Здравей! 😊".chars().collect();
    println!("{:x?}", "Здравей! 😊".as_bytes());
    println!("{:x?}", bytes);
    println!("{:?}", chars);

    for c in "Здравей! 😊".chars() {
        let c_string: String = c.to_string();
        let c_utf8 = c_string.as_bytes();

        println!("{}: code_point={:x}, utf8={:x?}", c, c as u32, c_utf8);
    }

    let len : usize = "Петко".chars().count();
    println!("Length of Петко is {}",len);
}
