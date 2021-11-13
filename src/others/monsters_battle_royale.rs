use proconio::input;

fn gcd(a:isize, b:isize) -> isize {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

fn main() {
  input! {
    n: usize,
    ms: [isize;n]
  }

  let mut next = ms[0];
  for i in 1..n {
    next = gcd(next, ms[i]);
  }
  println!("{}", next);
}