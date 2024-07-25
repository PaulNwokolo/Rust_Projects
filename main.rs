 /// This is mainly talking about tpye casting, basic arithmetic  and others 
 use std::io;

 fn main() {
    // doing some basic operations on this 
    let x = 127.0_f64;
    let y = 23.0_f64;

    let z = x + y;
    println!("{}", z);

    let z = x / y;
    println!("{}", z);

    let z = x - y;
    println!("{}", z);

    // type casting
     let a = 12.0f64;
     let b = 20.0f64;
     let c = a + b;
     println!("{}", c);


      // getting the input frorm the user
      let mut input = String::new();
      io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

      println!("Enter first value: ");
      let int_input : i64 = input
      .trim()
      .parse()
      .unwrap();
      println!("Enter the second value: ");
      let int_input_2 : i64 = input
      .trim()
      .parse()
      .unwrap();
      
     
}
