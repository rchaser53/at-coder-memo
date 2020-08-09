use proconio::input;
use proconio::marker::Usize1;

#[derive(Debug)]
struct Helper {
  pub map: Vec<Vec<(usize, usize)>>,
  pub result: Vec<usize>
}

impl Helper {
  pub fn traverse(
    &mut self,
    cur: usize,
    last_index: usize,
    last_id: usize
  ) {
    let mut id = 1;
    let targets = self.map.get(cur).unwrap().clone();
    for (to, index) in targets.iter() {
      let to = *to;
      let index = *index;
      if to == last_index { continue }
      if id == last_id { id += 1; }
      self.result[index] = id;
      id += 1;
      self.traverse(to, cur, self.result[index]);
    }
  }
} 

fn main() {
  let OVER = 1_000_000;
  input! {
    n: usize,
    edges: [(Usize1,Usize1);n-1]
  }

  let mut result: Vec<usize>  = vec![0;n-1];
  let mut map: Vec<Vec<(usize, usize)>> = vec![vec![];n];
  for (i, (from, to)) in edges.into_iter().enumerate() {
    map[from].push((to, i));
  }
  let mut helper = Helper { map, result };
  helper.traverse(0, OVER, OVER);
    
  let mut max = 0;
  for v in helper.result.iter() {
    max = std::cmp::max(max, *v);
  }
  println!("{}", max);

  for v in helper.result.iter() {
    println!("{}", v);
  }
}