use proconio::input;

fn main() {
  input! {
    K: usize,
    N: usize,
    mut arr: [usize;N]
  }

  let mut next = arr.clone().into_iter().map(|v| v + K).collect();
  let concated = [arr, next].concat();
  let mut result = usize::max_value();
  for i in (N-1)..(2*N) {
    result = std::cmp::min(result, concated[i] - concated[i - (N - 1)]);
  }

  println!("{}", result);
}
