// Convert temperatures between Fahrenheit and Celsius.


// fn main() {
//     let fahrenheit:f64 = 97.0;

//     let ans = fahrenheit_to_celsius(fahrenheit);

//     println!("The conversion to Celsius is: {:.2}", ans);

// }

// fn fahrenheit_to_celsius(f:f64) -> f64 {
//     (f-32.0)* (5.0/9.0)
// }



// Generate the nth Fibonacci number


fn main() {
   let n = 10;
   println!("The {}th Fibonacci number is: {}", n, fibonacci(n));
}

fn fibonacci(n: u32) -> u64 {
   if n == 0 {
    return 0;
   } else if n==1 {
    return 1;
   }

   let mut previous = 0;
   let mut current = 1;
   let mut position = 2;


   while position <=n {
    let next = previous + current;
    previous = current;
    current = next;
    position += 1;
   }

   current
}