use proconio::input;

fn culc(a: usize, b:usize) -> usize {
  if a % b == 0 {
    a / b
  } else {
    a / b + 1
  }
}

fn main() {
  input! {
    n: usize,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
  }

  let mut memo: Vec<usize> = vec![a,b,c,d,e];
  let mut min = usize::max_value();
  for (i, v) in memo.iter().enumerate() {
    min = std::cmp::min(min, *v);
  }
  let total = culc(n, min) + 4;
  
  println!("{}", total);
}