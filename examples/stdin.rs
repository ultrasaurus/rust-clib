use add::add;
use std::io;
use std::io::Write; 

fn get_i32() -> i32 {
  let mut input = String::new();
  print!("\nEnter a number: ");
  io::stdout().flush().expect("could not write to stdout");

  io::stdin().read_line(&mut input).expect("Not a valid string");
  input.trim().parse().expect(&format!("Expected a number, got {}", input))
}


fn main() {
  let a = get_i32();
  let b = get_i32();
  let sum = add::add(a,b);
  
  println!("\nTotal : {}\n\n", sum);

  // return(0);
}