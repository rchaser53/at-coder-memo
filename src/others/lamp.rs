use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
    mut rows: [String;h]
  }

  let mut columns: Vec<Vec<usize>> = vec![];
  let mut scores: Vec<Vec<usize>> = vec![];
  for row in rows {
    let mut score_row = vec![0;w];
    let mut count = 0;
    let mut index = 0;
    let chars = row.chars().collect::<Vec<char>>();
    for i in 0..w {
      match chars[i] {
        '.' => { count += 1; },
        '#' => { count = 0; },
        _ => panic!("nya-n")
      };
      score_row[i] = count;
    }
    let mut insert_v = 0;
    let mut result = vec![0;score_row.len()];
    score_row.reverse();
    for (i, v) in score_row.iter().enumerate() {
      let v = *v;
      if v == 0 {
        insert_v = 0;
      } else {
        insert_v = std::cmp::max(insert_v, v);
      };
      result[i] = insert_v;
    }
    columns.push(result);
    scores.push(score_row);
  }
   
  let mut rows = scores;
  for c in 0..w {
    let mut count = 0;
    for r in 0..h {
      let v = rows[r][c];
      let v = if v > 0 {
        count += 1;
      } else {
        count = 0;
      };
      rows[r][c] = count;
    }
    
    let mut r = h-1;
    let mut insert_val = 0;
    loop {
      let v = rows[r][c];
      let v = if v == 0 {
        insert_val = 0;
      } else {
        insert_val = std::cmp::max(insert_val, v);
      };
      rows[r][c] = insert_val;
      
      if r == 0 { break };
      r -= 1;
    }
  }
 
  let mut max = 0;
  for r in 0..h {
    for c in 0..w {
      max = std::cmp::max(max, columns[r][c] + rows[r][c]);
    }
  }
  
  if max == 0 {
    println!("0");
  } else {
    println!("{}", max - 1);
  }
}