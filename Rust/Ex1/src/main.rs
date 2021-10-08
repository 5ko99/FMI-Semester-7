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
}

fn add(x : u32, y : u32) -> u32 {
   return x + y;
}

fn print(x : u32) -> () {
	println!("{}",x);
}
