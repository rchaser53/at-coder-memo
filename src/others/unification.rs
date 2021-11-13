use proconio::input;

fn main() {
  input! {
    s: String
  }
  let origin = s.len();
  let mut s: Vec<char> = s.chars().collect::<Vec<char>>();
  
  let mut result = 0;
  let mut stack: Vec<char> = vec![s.remove(0)];
  
  while !s.is_empty() {
    let next = s.remove(0);
    if stack.is_empty() {
      stack.push(next);
    } else if stack.last().unwrap() != &next {
      stack.pop();
    } else {
      stack.push(next);
    }
  }
  println!("{}", origin - stack.len());
}