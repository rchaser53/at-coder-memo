use proconio::input;

fn gcm(a: usize, b: usize) -> usize {
  if b == 0 {
    a
  } else {
    gcm(b, a % b)
  }
}

fn main() {
  input! {
    n: usize,
    mut vals: [usize;n]
  }
  
  if vals.len() == 2 {
    println!(
      "{}",
      if vals[0] < vals[1] {
        vals[1]
      } else {
        vals[0]
      }
    );
    return
  }
  
  vals.sort();
  let mut memo: Vec<usize> = vec![0;n];
  memo[n-1] = vals[n-1];
  let mut index = vals.len() - 2;
  loop {
    memo[index] = gcm(vals[index], memo[index+1]);
    if index == 0 { break }
    index -= 1;
  }
  
  let mut max = 1;
  let mut m = vals[0];
  for i in 1..n-1 {
    max = std::cmp::max(max, gcm(m, memo[i+1]));
    m = gcm(m, vals[i]);
  }
  max = std::cmp::max(max, gcm(vals[1], memo[2]));
  max = std::cmp::max(max, m);
  
  println!("{}", max);
}