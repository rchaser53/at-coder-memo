use proconio::input;
 
fn main() {
  input! {
    x: isize,
    k: isize,
    d: isize
  }
  let times = x / d;
  if times.abs() > k {
    println!("{}", (x.abs() - (k * d).abs()).abs());
  } else {
    let left = x.abs() % d;
    if (k - times) % 2 == 0 {
      println!("{}", left.abs());
    } else {
      println!("{}", (left.abs() - d.abs()).abs());
    }
  }
}
