use proconio::input;
fn main(){
  input! {
    c: [[usize; 3]; 3]
  }
  let a = [c[0][0], c[1][0], c[2][0]];
  let b = [0, c[1][1]-a[1], c[2][2]-a[2]];
  for i in 0..3 {
    for j in 0..3 {
      if c[i][j] != a[i]+b[j] {
        println!("No");
        return;
      }
    }
  }
  println!("Yes");
}
