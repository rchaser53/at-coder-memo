use proconio::input;
use proconio::marker::Usize1;

fn traverse(
  result: &mut Vec<usize>,
  routes: &Vec<Vec<(usize, usize)>>,
  index: usize,
  val: usize,
  last_index: usize,
) {
  result[index] = val;
  for v in routes[index].iter() {
    if last_index == v.0 { continue }
    traverse(result, routes, v.0, val ^ v.1, index);
  }
}

fn main() {
  input! {
    n: usize,
    mut edges: [(Usize1, Usize1, usize);n-1]
  }
  let mut routes: Vec<Vec<(usize, usize)>> = vec![vec![];n];
  
  let edges: Vec<(usize, usize, usize)> = edges
  .into_iter()
  .map(|(a,b,c)| (a,b,c % 2))
  .collect();
  
  for (from, to, parity) in edges {
    routes[from].push((to, parity));
    routes[to].push((from, parity));
  }
  let mut result = vec![0;n];
  
  for v in routes[0].iter() {
    traverse(&mut result, &routes, v.0, 0 ^ v.1, 0);
  }
  
  for v in result {
    println!("{}", v);
  }
}