use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    ps: [f64;n]
  }
  let mut range = (0,k-1);
  let mut max = ps.iter().by_ref().take(k).sum::<f64>();
  let mut previous = max;
  for i in k..n {
    let val = previous - ps[i - k] + ps[i];
    if max < val {
      range = (i - k + 1, i);
      max = val;
    }
    previous = val;
  }
  
  let mut result = 0f64;
  for i in (range.0)..=(range.1) {
    result += (1f64 + ps[i]) / 2f64;
  }
  
  println!("{}", result);
}
