use proconio::input;
fn main(){
  input! {
    n: u32,
  }
  for i in 0..n {
    for j in 0..n {
      if (4*i)+(7*j) == n {
        println!("Yes");
        return;
      }
    }
  }
  println!("No");
}
