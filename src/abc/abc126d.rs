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

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize
) -> Vec<usize> {
  let mut score = vec![default_val;n];
  let mut pq = BinaryHeap::new();
  score[start] = 0;
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
    if score[u] < w_u {
      continue
    }
    
    for &(v, w_v) in graph[u].iter() {
      let w = w_u + w_v;
      if w < score[v] {
        score[v] = w;
        pq.push(std::cmp::Reverse((w, v)));
      }
    }
  }
  score
}
 
fn adjacency_list(
  n: usize,
  uvw: &Vec<(usize, usize, usize)>
) -> Vec<Vec<(usize, usize)>> {
  let mut e = vec![vec![]; n];
  for &(u, v, w) in uvw.iter() {
    e[u].push((v, w));
    e[v].push((u, w));
  }
  e
}
 
fn case_dijkstra() {
  input!{
    n:usize,
    vals:[(Usize1,Usize1,usize);n-1]
  }
  let inf = 10_000_000_000 * 1_000_000;
  let mut ad_list = adjacency_list(n, &vals);
  let mut result = dijkstra(n, inf, &ad_list, 0);
  
  for i in 1..n {
    if result[i] % 2 == 0 {
      result[i] = 0;
    } else {
      result[i] = 1;
    }
  }
  for v in result {
    println!("{}", v);
  }
}