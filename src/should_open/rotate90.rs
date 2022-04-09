fn rotate90(arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let n = arr.len();
  let mut result = vec![vec![0;n];n];
  for i in 0..n {
      for j in 0..n {
          result[j][n-1-i] = arr[i][j];
      }
  }
  result
}

pub fn sol(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
    let n = mat.len();
    for _ in 0..4 {
        let mut success = true;
        for i in 0..n {
            for j in 0..n {
                if mat[i][j] != target[i][j] {
                    success = false;
                }   
            }
        }
        if success {
            return true;
        }
        mat = rotate90(mat);
    }
    false
}