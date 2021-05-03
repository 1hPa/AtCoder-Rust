use proconio::input;
fn sum_digits (mut n: u32) -> u32 {
  let mut sum=0;
  while n > 0 {
    sum += n%10;
    n /= 10;
  }
  sum
}

fn main(){
  input! {
    n: u32,
    a: u32,
    b: u32,
  }
  let mut total=0;
  
  for i in 1..=n {
    let mut _i = i;
    let sum = sum_digits(_i);
    if sum >= a && sum <= b {
      total += i;
    }
  }
  println!("{}", total)
}
