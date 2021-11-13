use proconio::input;

fn main() {
  input! {
    N: usize,
    M: usize,
    X: usize
  }
  
  let mut books: Vec<Vec<usize>> = Vec::with_capacity(M);
  for i in 0..N {
    input! {
      C: [usize;M+1],
    }
    books.push(C.into_iter().collect());
  }
  
  let mut min = usize::max_value();
  for i in 0..(1 << N) {
    let mut result = vec![0;M+1];
    let mut total = 0;
  	for j in 0..N {
      if (i >> j) & 1 == 0 { continue }
      total += books[j][0];
      for k in 1..M+1 {
        result[k] += books[j][k];
      }
    }
	
    let mut flag = true;
    for j in 1..M+1 {
      if result[j] < X {
        flag = false;
        break;
      }
    }
    
    if flag {
      min = std::cmp::min(min, total);
    }
  }
  
  if min == usize::max_value() {
    println!("-1");
  } else {
    println!("{}", min);
  }
}