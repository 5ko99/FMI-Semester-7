fn main() {
    //default rust uses move semantics for strings(mutable)
    let mut a = 2;
    let mut b = a;
    println!("{}",a);

    //clone
    let s1 = String::from("abc");
    let s2 = s1.clone();

    //copy - bitwise copying


    //giving reference
    let len = calculateLen(&s1);

    // you can have only one immutable refernce
    //mutable -> mutable,  mutable -> immutable, mutable -x> immutable no way
    //one mutable XOR many immutable
    let mut mutStr = String::from("abc");

    //slices
        let s = String::from("hello, world");
        let r1 = &s[..];
        let r2 = &r1[1..4];
        println!("{} \n{}", r1,r2);

    //vectors
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);
    let v = vec![1, 2, 3];

    let slice = &v[1..3];
    println!("{:?}",slice);
}

fn calculateLen(s : &String) -> usize {
    s.len()
}

fn change(s : &mut String) {
    s.push_str(", is me Petko")
}
