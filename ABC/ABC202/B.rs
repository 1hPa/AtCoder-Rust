use proconio::input;
fn main(){
  input! {
    s: String
  }
  println!("{}", rot(s));
}

fn rot(s: String) -> String {
  s.chars().rev().map(|c| match c {
      '6' => '9',
      '9' => '6',
      c => c
    }).collect()
}
