use proconio::input;
fn main(){
  input! {
    a: u32,
    b: u32,
  }
  let mut cnt=0;
  for i in a..=b {
    if i/10000 == i%10 && i/10%10 == i/1000%10 {
      cnt += 1;
    }
  }
  println!("{}", cnt);
}
