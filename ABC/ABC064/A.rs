use proconio::input;
fn main(){
  input! {
    r: i32,
    g: i32,
    b: i32,
  }
  let sum;
  sum = r*100 + g*10 + b;
  if sum%4 == 0 {
    println!("YES");
  } else {
    println!("NO");
  }
}
