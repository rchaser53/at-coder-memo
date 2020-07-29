use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize;n]
  }

  let mut min = a[0];
  let mut count = 0;
  for i in 0..n {
    let val = a[i];
    if min >= val {
      min = val;
      count += 1;
    }
  }
  println!("{}", count);
}
