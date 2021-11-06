//lifetime parameter in strucutres
#[derive(Debug)]
struct Words<'a> {
    text: Option<&'a str>,
}

impl<'a> Words<'a> {
    fn new(text: &'a str) -> Self {
        Words { text: Some(text) }
    }

    fn next_word(&mut self) -> Option<&'a str> {
        let text : &str = self.text?;
        let mut iter = text.splitn(2, char::is_whitespace);

        match (iter.next(), iter.next()) {
            (Some(word), rest) => {
                self.text = rest;
                Some(word)
            },
            _ => unreachable!()
        }
    }
}
trait MyTrait {}
impl<'a> MyTrait for &'a String {}

struct Wrapper<T: MyTrait>(T);

fn save_for_later<T: MyTrait + 'static>(something: T) -> Wrapper<T> {
    Wrapper(something)
}

fn main() {
    let x = 5;

    let r2 = {
        let r1 = &x;


        &*r1
    };

    let s1 = String::from("дългият низ е дълъг");
    {
        let s2 = String::from("къс низ");
        let result = longer(&s1, &s2);
    }

    println!("{}",s1);

    println!("{}", r2);

    //static lives for the entire program's life
    let s: &'static str = "I have a static lifetime.";

    let mut words = Words::new("hello world");
    println!("{:?}", words.next_word());
    println!("{:?}", words.next_word());
    println!("{:?}", words.next_word());
}

/// Връща по-дългия от двата низа -- compilation error without lifetime annotation
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
