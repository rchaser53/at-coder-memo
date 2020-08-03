use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    return a
  }
  gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
}

fn main() {
  input! {
    a: usize,
    b: usize
  }
  
  println!("{}", lcm(a, b));
}
