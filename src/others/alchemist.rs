use proconio::input;

fn main() {
  input! {
    n: usize,
    mut v: [usize;n]
  }
  v.sort();
  let mut v: Vec<f64> = v.into_iter().map(|v| v as f64).collect();
  
  let mut val = v.remove(0);
  for vv in v {
    val = (vv + val) / 2f64;
  }
  println!("{}", val);
}