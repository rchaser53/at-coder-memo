use proconio::input;

fn gcv(a: isize, b:isize) -> isize {
  if b == 0 {
    a
  } else {
    gcv(b, a % b)
  }
}

fn lcm(a: isize, b:isize) -> isize {
  a * b / gcv(a, b)
}

fn main() {
  input! {
    a: isize,
    b: isize,
    c: isize,
    d: isize
  }
  
  let a = a - 1;
  let common = lcm(c, d);
  let result = b - (b/c + b/d - b/common) - (a - (a/c + a/d - a/common));
  println!("{}", result);
}