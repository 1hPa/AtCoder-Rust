use proconio::input;
fn main(){
  input! {
    n: u32,
  }
  for i in 1..10 {
    for j in 1..10 {
      if i*j == n {
        println!("Yes");
        return;
      }
    }
  }
  println!("No");
}
