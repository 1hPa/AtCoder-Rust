use proconio::input;
fn main(){
  input! {
    n: usize,
    a: [usize; n]
  }
  let mut b = vec![0; 200];
  for i in a {
    b[i.rem_euclid(200)] += 1;
  }
  let mut ans: i64 = 0;
  for j in b {
    ans += j*(j-1);
  }
  println!("{}", ans/2);
}
