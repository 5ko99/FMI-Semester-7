//Converting in rust
use std::fmt::{Debug, Display};
use std::{io, str};
use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use std::fs;

#[derive(Debug,Copy,Clone)]
struct Celsius(f64);
#[derive(Debug,Copy,Clone)]
struct Fahrenheit(f64);
#[derive(Debug)]
struct Kelvin(f64);



impl From<Celsius> for Kelvin {
    fn from(t: Celsius) -> Kelvin { Kelvin(t.0 + 273.15) }
}

impl From<Fahrenheit> for Celsius {
    fn from(t: Fahrenheit) -> Celsius { Celsius((t.0 - 32.0) / 1.8) }
}

impl From<Fahrenheit> for Kelvin {
    fn from(t: Fahrenheit) -> Kelvin { Kelvin::from(Celsius::from(t)) }
}

fn energy_to_heat_water<T1, T2>(from: T1, to: T2, mass: f64) -> f64
    where
        T1: Into<Kelvin>,
        T2: Into<Kelvin>
{
    let from = from.into();
    let to   = to.into();
    from.0 * to.0 * mass
}

//how to implement from str for our method
#[derive(Debug)]
struct Potato {
    is_a_potato: bool,
}

impl FromStr for Potato {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "картоф" {
            Ok(Potato { is_a_potato: true })
        } else {
            Err(String::from("what is this even"))
        }
    }
}

//Working with files and error handling
//Macro
macro_rules! _try {
    ($expr:expr) => {
        match $expr {
            Ok(result) => result,
            //Err(e) => return Err(e),
            Err(e) => return Err(e.into())
        }
    }
}

//Custom errors
struct FancyError { message: String }

impl From<io::Error> for FancyError {
    fn from(e: io::Error) -> Self {
        FancyError { message: format!("IO Error: {}", e) }
    }
}




fn main() {
    let temp = Fahrenheit(75.0);
    let temp_1 : Celsius = temp.into();
    println!("{:?}°C is {:?}°F", &temp, &temp_1);
    let e = energy_to_heat_water(Celsius(20.5), Celsius(100.0), 1.0);
    println!("{}",e);

    use std::str::FromStr;
    let x : Result<isize,_> = "-13".parse();
    println!("{:?}",x.unwrap());

    let p1: Result<Potato, _> = "картоф".parse();
    let p2: Result<Potato, _> = "патладжан".parse();

    println!("{:?}\n{:?}", p1, p2);

    //files
    match all_your_quotes_are_belong_to_us() {
        Ok(contents) => println!("{}", contents),
        Err(e) => panic!("😞 {}", e),
    }

    //error handaling
    let number = "foo".parse::<i32>().unwrap_or_else(|e| {
        println!("Warning: couldn't parse: {}", e);
        0
    });

}

fn all_your_quotes_are_belong_to_us() -> Result<String, io::Error> {
    let mut deep = _try!(File::open("deep_quotes.txt"));
    //let mut wide = _try!(File::open("wide_quotes.txt"));

    let mut contents = String::new();
    _try!(deep.read_to_string(&mut contents));
    // _try!(wide.read_to_string(&mut contents));
    let mut quotes = String::new();
    contents.push_str(&fs::read_to_string("deep_quotes.txt")?);
    Ok(contents)
}
