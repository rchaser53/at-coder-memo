use proconio::input;

fn f1(a:usize, b:usize) -> usize {
  if a == 0 {
    0
  } else {
    a/b + f1(a/b, b)
  }
}

fn f2(a:usize, b:usize) -> usize {
  if a % 2 == 1 {
    return f1(a, b) - f2(a-1, b);
  }
  let mut result = f1(a/2, b);
  if b == 2 {
    result += a/2;
  }
  result
}

fn main() {
  input!{
    mut n: usize,
  }
    
  println!("{}", std::cmp::min(f2(n, 2), f2(n, 5)));
}
