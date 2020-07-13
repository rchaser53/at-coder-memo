use proconio::input;
fn main() {
  input! {
    S: String,
  }

  let mut counts: [usize;2019] = [0;2019];
  let mut total = 0;
  let mut result = 0;
  let mut target_str: Vec<char> = S.chars().collect();
  let mut x = 1;
  target_str.reverse();
  
  for v in target_str.iter() {
    counts[total] += 1;
    let val = v.to_string().parse::<usize>().unwrap();
    total = (total + val * x) % 2019;
    result += counts[total];
    x = x * 10 % 2019;
  }

  println!("{}", result);
}