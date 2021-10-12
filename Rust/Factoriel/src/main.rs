fn factorial(n : u128) -> u128 {
    if n<=1 {
        return 1;
    } else {
        factorial_help(n,1)
    }
}

fn factorial_help(n : u128, fact : u128) -> u128 {
    if n<=1 {
        return fact;
    } else {
        factorial_help(n-1,n*fact)
    }
}

fn main() {
    let n :u128 = 4;
    let result : u128 = factorial(n);
    println!("{}! = {}",n,result);
}
