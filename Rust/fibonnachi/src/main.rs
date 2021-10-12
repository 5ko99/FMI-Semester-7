pub fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    let n = 20;
    let result = fib(n);
    println!("{}-th number of Fibonacci is : {}",n,result);
}
