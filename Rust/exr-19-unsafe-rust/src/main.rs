use std::os::raw::c_int;

#[link(name="math")]
extern {
    fn add_in_c(a: c_int, b: c_int) -> c_int;
}


fn main() {
    let res = unsafe { add_in_c(1, 2) };
    println!("{}", res);
}
