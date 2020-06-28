#[warn(non_snake_case)]
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim_end().parse().unwrap();

  	let mut last: Vec<i32> = vec![0, 0, 0];
  
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let a:Vec<i32> = s
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();      
      
     	let t = a[0] - last[0];
     	let x = (a[1] - last[1]).abs();
     	let y = (a[2] - last[2]).abs();
        let sum = x + y;
      	if t - sum < 0 || sum % 2 != t % 2 {
      		println!("No");
            return
        }
        
        last = a;
    }
    
	println!("Yes");
}
