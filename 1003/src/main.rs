use std::io::{stdin, Read};

static mut MEMO: Vec<(usize, usize)> = Vec::new();

unsafe fn fibo(n: usize) -> (usize, usize) {
  if n < MEMO.len() {
    return MEMO[n];
  }

  let (a, b) = fibo(n - 1);
  let (c, d) = fibo(n - 2);
  let result = (a + c, b + d);
  MEMO.push(result);
  return result;
}

fn main() {
  let mut input = String::new();
  stdin().read_to_string(&mut input).unwrap();
  let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

  unsafe {
    MEMO.push((1, 0));
    MEMO.push((0, 1));
  }

  let counts = input.next().unwrap();
  for _ in 0..counts {
    let nth = input.next().unwrap();
    let counts: (usize, usize) = unsafe { fibo(nth) };
    println!("{} {}", counts.0, counts.1);
  }
}

// 모듈러
//
// fn fib(nth: usize) -> (u64, u64) {
//   if nth == 0 {
//     return (0u8.into(), 1u8.into());
//   }
//   let modulo_rem = nth % 2;
//   let (a, b): (u64, u64) = fib((nth - modulo_rem) / 2);
//
//   // F(2n) = F(n) * (2F(n+1) - F(n))
//   // F(2n+1) = F(n+1)^2 + F(n)^2
//   println!("{} {}", a, b);
//   let big_a = a as u128;
//   let big_b = b as u128;
//   // let a = util_mod(big_a);
//   // let b = util_mod(big_b);
//
//   println!("BIG_B*2: {} BIG_A: {}", big_b * 2, big_a);
//   let c_partial = util_mod(big_b * 2 - big_a);
//   let c = util_mod(big_a * c_partial as u128);
//   let d = ((big_a * big_a + big_b * big_b) % 1_000_000_000) as u64;
//
//
//   if modulo_rem == 1 {
//     let summed = (c + &d) % 1_000_000_000;
//     return (d, summed);
//   } else {
//     return (c, d);
//   }
// }
