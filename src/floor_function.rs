use proconio::input;

fn main() {
  input! {
    A: f64,
    B: f64,
    N: f64
  }

  if B - 1f64 < N {
    println!("{}", (A * (B - 1f64) / B).floor());
  } else {
    println!("{}", (A * N / B).floor());
  }
}
