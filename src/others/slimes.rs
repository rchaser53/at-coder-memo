use proconio::input;

fn main() {
  input! {
    n: usize,
    s: String
  }
  let mut s: Vec<char> = s.chars().collect();
  let mut index = 0;
  
  while index + 1 < s.len() {
    if s[index] == s[index + 1] {
      s.remove(0);
    } else {
      index += 1;
    }
  }
  println!("{}", s.len());
}