use proconio::input;
fn main(){
  input! {
    n: usize,
    mut d: [usize; n]
  }
  let mut count = 1;
  d.sort();
  for i in 1..n {
    if d[i-1] != d[i] {
      count += 1;
    }
  }
  println!("{}", count);
}
