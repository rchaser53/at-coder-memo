use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    N: usize,
    M: usize,
    mut H: [usize;N]
  }
  
  let mut map = vec![true;N];
  
  for _ in 0..M {
  	input! {
      A: Usize1,
      B: Usize1
    }
    if *map.get(A).unwrap() {
      map[A] = H[A] > H[B];
    }

    if *map.get(B).unwrap() {
      map[B] = H[B] > H[A];
    }
  }
  
  let mut count = 0;
  for v in map {
  	if v {
      count += 1;
    }
  }
  println!("{}", count);
}