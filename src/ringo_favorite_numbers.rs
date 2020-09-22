use proconio::input;
 
fn main() {
  input! {
    d: u32,
    n: usize
  }
  if n == 100 {
    println!("{}", (n + 1) * 100usize.pow(d));
  } else {
    println!("{}", n * 100usize.pow(d));  
  }
}