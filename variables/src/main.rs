// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("Hi, {}", THREE_HOURS_IN_SECONDS);
// }


// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");

//     let spaces = "    ";
//     let spaces = spaces.len();


//     println!("The value of spaces is: {spaces}");
// }


   // fn main() {
   //  let guess: u32 = "42".parse().expect("Not a number");
   //  println!("The guess number is {}", guess);
   //  }   


   // fn main() {
   //  let c = 'z';
   //  let z: char = 'ℤ'; // with explicit type annotation
   //  let heart_eyed_cat = '😻';

   //  println!("the value of c is {}", c);
   //  println!("the value of z is {}", z);
   //  println!("the value of heart eyed cat is {}", heart_eyed_cat);

   //  }   



   // fn main() {
   //  let tup: (i32, f64, u8) = (500, 6.4, 1);
   //  let (x, y, z) = tup;

   //  println!("the value of y is: {x}");
   //  println!("the value of y is: {y}");
   //  println!("the value of y is: {z}");

   // }

   // fn main() {
   //  let x: (i32, f64, u8) = (500, 6.4, 1);

   //  let five_hundred = x.0;

   //  let six_point_four = x.1;

   //  let one = x.2;

   //  println!("the value of five_hundred is {five_hundred}", );
   //  println!("the value of six_point_four is {six_point_four}", );
   //  println!("the value of one is {one}", );

   //  }


   // fn main() {
   //  let a = [1,2,3,4,5];

   //  let first = a[0];
   //  let second = a[1];

   //  println!("the value of first is {first}", );
   //  println!("the value of second is {second}", );

   // }



  fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

































