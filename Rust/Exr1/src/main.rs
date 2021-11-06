fn main() {
   for i in 1..10 {
      println!("{}",i);
   }
   let arr : [bool;3] = [true,true,false];
   for i in arr {
      println!("{}",i);
   }
   let empty : () = ();

   let myTupple : (i8,bool) = (4i8,true);

   println!("First value:{}\tSecond value:{}",myTupple.0,myTupple.1);

   let x = add(3,5);
   let x = {
      let a = 3;
      let b = 4;
      a + b
   };
   println!("{}",x);

   let mut z : u8 = 5;
   z+=4;
   println!("Mutable variable z={}",z);

   let s: &str = "Нещо друго";
   let heart1: char = '❤';
   println!("String s= {} and a heart {}",s,heart1);

   //arrays
   let nested: [[i32; 3]; 2] = [
      [1, 2, 3],
      [4, 5, 6],
   ];
   let tuple: (i32, u32, bool) = (1, 2, false);
   for i in nested {
      for j in i {
         print!("{} ",j);
      }
      println!();
   }

   //Types conversion
   let a : isize = 256;
   let b : i32 = 2;
   let result : f64 = a as f64 / b as f64;
   println!("Result={}",result);

   let sometingLikeFunction = {
      let a = 1;
      let b = 2;
      a + b
   };
   //Ternary operator
   let bigger : isize = if a > b as isize { a as isize } else { b as isize };
   println!("Bigger={}",bigger);

   //Lambda- closures
   let multiply = |a, b| {a*b};
   println!("5*6 = {}",multiply(5,6));
}

fn add(x : u32, y : u32) -> u32 {
   return x + y;
}

fn print(x : u32) -> () {
	println!("{}",x);
}
