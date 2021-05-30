use proconio::input;
fn main(){
  input! {
    n: usize,
    k: usize
  }
  let mut sum = 0;
  for i in 1..=k {
    for j in 1..=n {
      sum += j*100 + i;
    }
  }
  println!("{}", sum);
}
