use proconio::input;

fn main(){
  input!{
    N: isize, X: isize,
    S: String,
  }
  let mut ans = X;
  for i in S.chars(){
    if i=='o'{
      ans += 1;
    }else if i=='x'{
      if ans>0{
        ans -=1;
      }
    }
  }
  println!("{}", ans);
}
