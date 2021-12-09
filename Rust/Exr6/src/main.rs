//generics

use std::fmt::{Debug, Display};

struct Point<T> {
    x: T,
    y: T,
}

enum Message<T, A> {
    Text(T),
    Action(A),
}

//traits

impl<T> Point<T> where T: Display {
    fn print(&self) -> () {
        println!("(x,y)=({},{})",self.x,self.y);
    }
}

trait ToJson {
    //you can give default implementation to trait
    fn to_json(&self) -> String {
        String::from("null")
    }
}

impl ToJson for String {
    fn to_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl ToJson for i32 {
    fn to_json(&self) -> String {
        format!("{}", self)
    }
}

impl ToJson for f32 {
    fn to_json(&self) -> String {
        format!("{}", self)
    }
}

impl ToJson for () {}

impl<T> ToJson for Option<T> where T: ToJson {
    fn to_json(&self) -> String {
        match self {
            Some(val) => val.to_json(),
            None => String::from("null"),
        }
    }
}

impl<T> ToJson for Vec<T> where T: ToJson {
    fn to_json(&self) -> String {
        let mut iter = self.iter();
        let first = iter.next();

        let mut result = match first {
            Some(first) => first.to_json(),
            None => String::new(),
        };

        for e in iter {
            result.push_str(", ");
            result.push_str(&e.to_json());
        }

        format!("[{}]", result)
    }
}

//You can define a trait for own type
struct Student {
    age: i32,
    full_name: String,
    number: i32,
    hobby: Option<String>
}

impl ToJson for Student {
    fn to_json(&self) -> String {
        format!(
            r#"{{
    "age": {},
    "full_name": {},
    "number": {},
    "hobby": {}
}}"#,
            self.age.to_json(), self.full_name.to_json(),
            self.number.to_json(), self.hobby.to_json()
        )
    }
}

fn to_json<T: ToJson>(value: T) -> String {
    value.to_json()
}

//you can implement two traits
fn log_json_transformation<T: ToJson + Debug>(value: T) {
    println!("{:?} -> {}", value, value.to_json());
}

fn to_json1(value: &dyn ToJson) -> String {
    value.to_json()
}

//This is calculated at compiletime
const A: usize = 1;
const fn identity<T>(value: T) -> T { value }

//graph trait
struct Node;
struct Edge;
struct MyGraph;

trait Graph {
    type N;
    type E;

    fn has_edge(&self, node1: &Self::N, node2: &Self::N) -> bool;
    fn edges(&self, node: &Self::N) -> Vec<Self::E>;
}

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    }

    fn edges(&self, n: &Node) -> Vec<Edge> {
        Vec::new()
    }
}

fn distance<G: Graph<N=Node, E=Edge>>(graph: &G, start: &G::N, end: &G::N) -> u32 {
    5 as u32
}

//Add trait
trait Add<RHS> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add<f32> for f64 {  // Имплементирай ми "добавяне на f32 към f64"
    type Output = f64;  // Като резултата ще е винаги f64
    fn add(self, rhs: f32) -> Self::Output {
        (self +  rhs as f64) as f64
    }
}

impl Add<f64> for f32 {  // Имплементирай ми "добавяне на f64 към f32"
type Output = f64;  // Като резултата ще е винаги f64
fn add(self, rhs: f64) -> Self::Output {
    (self as f64 +  rhs as f64) as f64
}
}


fn main() {
    let float = Point{x: 1.25, y:3.8};
    let student = Student{age:22,full_name: String::from("Petko"), number: 45546, hobby: Some(String::from("Playing guitar."))};
    let arr= vec![Some(2),Some(3),None,Some(5),];
    let result = to_json(arr);
    println!("{}",result);
    float.print();

    //traits
    let trait_object: &dyn ToJson = &5;
    println!("{}",to_json1(trait_object));

    //How to make inhomogeneous
    use std::fmt::Debug;
    let non_hom_vector = vec![&5 as &dyn Debug,&5.5,&'P'];
    println!("{:?}", non_hom_vector);

    let result = (5.2 as f64).add(6.8 as f32);
    println!("{}",result);
}
