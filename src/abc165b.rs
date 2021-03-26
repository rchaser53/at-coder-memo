use proconio::input;

fn main() {
  input! {
    x:usize
  }
  
  let mut count = 0;
  let mut val = 100;
  while val < x {
    val += val / 100;
    count += 1;
  }
  println!("{}", count);
}