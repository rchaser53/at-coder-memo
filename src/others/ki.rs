use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

fn dfs(
  map: &Vec<Vec<usize>>,
  scores: &mut Vec<usize>,
  index: usize,
  last: usize
) {
  for next in map[index].iter() {
    let next = *next;
    if last == next { continue }
    scores[next] += scores[index];
    dfs(map, scores, next, index);
  }
}

#[fastout]
fn main() {
  input! {
    n: usize,
    q: usize,
    trees: [(Usize1, Usize1);n-1],
    points: [(Usize1, usize);q]
  }
  let mut scores: Vec<usize> = vec![0;n];
  let mut map: Vec<Vec<usize>> = vec![vec![];n];
  let NotMatchIndex = n * 10;
  
  for (from, to) in trees {
    map[from].push(to);
    map[to].push(from);
  }

  for (root, point) in points {
    scores[root] += point;
  }
  
  dfs(&map, &mut scores, 0, NotMatchIndex);  

  println!("{}", scores
    .into_iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>()
    .join(" ")
  );
}