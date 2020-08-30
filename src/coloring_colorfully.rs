use proconio::input;

fn main() {
  input! {
    s: String,
  }
  let s = s.chars().collect::<Vec<char>>();
  
  let mut b_count = 0;
  let mut c_count = 0;
  
  for (i, c) in s.iter().enumerate() {
    let c = *c;
    if i % 2 == 0 && c == '0' {
      b_count += 1;
    } else if i % 2 == 1 && c == '1' {
      b_count += 1;
    }
  }
  
  for (i, c) in s.iter().enumerate() {
    let c = *c;
    if i % 2 == 0 && c == '1' {
      c_count += 1;
    } else if i % 2 == 1 && c == '0' {
      c_count += 1;
    }
  }
  
  if c_count < b_count {
    println!("{}", c_count);
  } else {
    println!("{}", b_count);
  }
}