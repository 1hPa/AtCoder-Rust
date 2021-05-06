use proconio::input;
fn main(){
  input! {
    n: usize,
    _: usize,
    mut s: [String; n]
  }
  s.sort();
  for i in s {
    print!("{}", i);
  }
  println!("");
}
