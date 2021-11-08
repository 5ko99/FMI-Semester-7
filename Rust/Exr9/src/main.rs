struct MagicTrick {
    description: String,
    secrets: Vec<String>,
    skills: Vec<String>,
}

use std::fmt::{self, Display, Formatter, Debug};

impl Display for MagicTrick {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Магически трик {:?}", self.description)
    }
}

impl Debug for MagicTrick {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write! {
            f,
            r#"Трик
Описание: {:?}
Тайни: {:?}
Умения: {:?}"#,
            self.description,
            self.secrets,
            self.skills
        }
    }
}

struct OneTwoThree {
    state: u8,
}

impl OneTwoThree {
    fn new() -> Self {
        Self { state: 0 }
    }
}

impl Iterator for OneTwoThree {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state < 3 {
            self.state += 1;
            Some(self.state)
        } else {
            None
        }
    }
}

fn main() {
    let trick = MagicTrick {
        description: String::from("Изчезваща монета"),
        secrets: vec![String::from("Монетата се прибира в ръкава")],
        skills: vec![String::from("Бързи ръце"), String::from("Заблуда")],
    };
    println!("{}", trick);
    println!("===");
    println!("{:?}", trick);

    let mut iter = OneTwoThree::new();

    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    //iter and iter mut-> reference
    let mut v = vec![1,2,3];
    for i in v.iter() {
        println!("{}",i)
    }
    for i in v.iter_mut() {
        *i+=1;
    }
    println!("{:?}", v);
    let iter = v.into_iter().map(|x| x *x *x).filter(|&x| x < 40);
    for i in iter {
        println!("{}", i);
    }
    //It can be made to vector also
    let v = vec![2,3,4];
    let v = v
        .into_iter()
        .map(|x|x+5 )
        .filter(|&x| x < 9)
        .collect::<Vec<i32>>();
    println!("{:?}", v);

    //Closure can use upper defined variables by reference by default
    let other = String::from("foo");
    Some("bar").map(|s| s.len() + other.len());
    //There are also move closures
    let f = move |s: String| s.len() + other.len();

    //Fn traits are used to mark if something is closure
    println!("{}", eval_and_increment(|| 1));

    //Curring
    println!("{}",curry(1)(3));

    for x in create_iter() {
        println!("{:?}", x);
    }
}

fn eval_and_increment<F>(f: F) -> usize where F: FnOnce() -> usize {
    f() + 1
}

//Returning closure
fn curry(a: u32) -> impl Fn(u32) -> u32 {
    move |b| a + b
}

//This way you can return iterators
fn create_iter() -> impl Iterator<Item = impl Debug> {
    vec![0, 1, 2].into_iter().map(|x| x * 2).enumerate()
}