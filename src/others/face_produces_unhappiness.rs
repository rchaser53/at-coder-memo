use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    s: String
  }
  let mut chars: Vec<char> = s.clone().chars().collect();
  
  let mut compresses: Vec<usize> = vec![];
  let mut consective = 1;
  let mut last = chars.remove(0);
  for c in chars {
    if c == last {
      consective += 1;
    } else {
      compresses.push(consective);
      last = c;
      consective = 1;
    }
  }
  compresses.push(consective);
  let len = compresses.len();
  if len <= k * 2 {
    println!("{}", s.len() - 1);
    return
  }
  
  let mut result = k * 2;
  for v in compresses {
    result += v - 1;
  }
  
  println!("{}", result);
}