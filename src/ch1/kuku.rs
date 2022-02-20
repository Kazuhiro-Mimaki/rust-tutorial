fn main() {
  for y in 1..10 {
    for x in 1..10 {
      println!("{:3},", x * y);
    }
    println!("");
  }
}
