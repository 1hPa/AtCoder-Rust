use proconio::input;

fn main(){
  input! {
    n: usize,
    mut mount: [(String, usize); n],
  }
  mount.sort_by_key(|v| v.1);
  println!("{}", mount[n-2].0)
}
