use proconio::input;
fn main(){
  input! {
    a: usize,
    b: usize,
    c: usize
  }
  if a == b {
  	println!("{}", c);
    return;
  }
  if b == c {
  	println!("{}", a);
    return;
  }
  if c == a {
  	println!("{}", b);
    return;
  }
  else {
  	println!("{}", 0);
  }
}
