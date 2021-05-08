use proconio::input;
fn main(){
  input! {
    mut n: usize,
    k: usize
  }
  for i in 0..k {
    if n%200 == 0 {
      n = n/200;
    } else {
      n = n*1000 + 200;
    }
  }
  println!("{}", n);
}
