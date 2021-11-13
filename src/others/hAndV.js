function extractVal(input) {
  input = input.split("\n");
  const [h,w,k] = input.shift().trim().split(" ").map(v => parseInt(v, 10));
  const rows = new Array(h);
  for (let i=0;i<h;i++) {
  	rows[i] = input[i].trim().split("");
  }
  
  let blacks = 0;
  for (let i=0;i<rows.length;i++) {
  	for (let j=0;j<rows[0].length;j++) {
    	blacks += rows[i][j] === "#" ? 1 : 0;
    }
  }
 
  return {
    h, w, k, rows, blacks
  }
}
 
const copy = (arr) => {
  const rows = new Array(arr.length);
  for (let i=0;i<rows.length;i++) {
    rows[i] = [...arr[i]];
  }
  return rows;
}
 
function Main(input) {
  const {
    h, w, k, rows, blacks
  } = extractVal(input);
  
 
  
  if (blacks < k) {
  	console.log(0);
    return
  }
  
  let total = 0;
  const paintRow = (arr, r) => {
    let black = 0;
    for (let i=0;i<arr[0].length;i++) {
      if (arr[r][i] === "#") {
        black += 1;
      }
      arr[r][i] = "r";
    }
    return black;
  }
 
  const paintColumn = (arr, c) => {
    let black = 0;
    for (let i=0;i<arr.length;i++) {
      if (arr[i][c] === "#") {
        black += 1;
      }
      arr[i][c] = "r";
    }
    return black;
  }
 
  const traverse = (arr, r, c, left) => {    
      if (r < arr.length) {
        const new1 = copy(arr);
        traverse(new1, r+1, c, left - paintRow(new1, r));
        traverse(arr, r+1, c, left);
      } else if (c < arr[0].length) {
        const new3 = copy(arr);
        traverse(new3, r, c+1, left - paintColumn(new3, c));
        traverse(arr, r, c+1, left);
      } else {
      	if (left == k) total++
      }
  }
 
  traverse(rows, 0, 0, blacks);
  
  console.log(total);
}
//*この行以降は編集しないでください（標準入出力から一度に読み込み、Mainを呼び出します）
Main(require("fs").readFileSync("/dev/stdin", "utf8"));