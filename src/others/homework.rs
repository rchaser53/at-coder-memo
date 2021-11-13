use proconio::input;

fn main() {
  input! {
    mut n: isize,
    m: usize,
    vals: [isize;m]
  }
  
  for v in vals {
    n -= v;
  }
  
  if n < 0 {
    println!("-1");  
  } else {
    println!("{}", n);
  }
}