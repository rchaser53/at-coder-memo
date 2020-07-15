use proconio::input;

fn culc(a: usize, b:usize) -> usize {
	if b == 0 {
      return a
    }
    culc(b, a % b)
}

fn main() {
  input! {
    K: usize
  }
  
  let mut map: Vec<[usize;201]> = Vec::with_capacity(201);
  for _ in 0..=200 {
    map.push([0;201]);
  } 
  let mut result = 0;
  
  for i in 1..=200 {
    for j in 1..=200 {
      map[i][j] = culc(i, j);
    }
  }
  
  for i in 1..=K {
    for j in 1..=K {
      let a = map[i][j];
      for k in 1..=K {
        result += map[a][k];
      }
    }
  }
  
  println!("{}", result);
}