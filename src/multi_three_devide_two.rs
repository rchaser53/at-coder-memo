use proconio::input;

fn helper(mut a: usize) -> usize {
  if a <= 1 { return 0 }
  let mut count = 0;
  while 0 < a && a % 2 == 0 {
    a /= 2;
    count += 1;
  }
  count
}

fn main() {
  input! {
    n: usize,
    vals: [usize;n]
  }
  
  let mut result = 0;
  for v in vals {
    result += helper(v);
  }
  println!("{}", result);
}