use proconio::input;

fn main() {
  input! {
    n: usize,
    mut ds: [usize;n],
  }
  ds.sort(); 
  let k = ds.len() / 2;
  println!("{}", ds[k] - ds[k-1]);
}