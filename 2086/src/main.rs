use std::io::{stdin, Read};

// 행렬곱셈, 2x2 행렬.. 
fn multiply_matrix(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
  let mut result = [[0u64; 2]; 2];
  for i in 0..2 {
    for j in 0..2 {
      for k in 0..2 {
        result[i][j] += (((a[i][k] as u128 % 10_000_000_000) * (b[k][j] as u128 % 10_000_000_000)) % 10_000_000_000) as u64;
      }
    }
  }
  return result;
}

// 행렬 거듭제곱
fn pow_matrix(mut a: [[u64; 2]; 2], mut n: usize) -> [[u64; 2]; 2] {
  let mut result = [[1u64, 0u64], [0u64, 1u64]];
  while n > 0 {
    if n % 2 == 1 {
      result = multiply_matrix(result, a);
    }
    a = multiply_matrix(a, a);
    n /= 2;
  }
  return result;
}

// 피보나치 수열 계산 ( 행렬 거듭제곱)
fn fib(nth: usize) -> u64 {
  if nth == 0 {
    return 0;
  }
  let mut matrix = [[1u64, 1u64], [1u64, 0u64]];
  matrix = pow_matrix(matrix, nth - 1);
  return matrix[0][0];
}
unsafe fn unsafe_fibo(n: usize) -> usize { fib(n) as usize }

// NOTE: 짝수항의 합은, F(2n) = F(2n+2) - 1
// 홀수항의 합은, F(2n+1) = F(2n+2) - 1
fn fibo(n: usize) -> usize {
  if n % 2 == 1 {
    return unsafe { unsafe_fibo((n / 2) * 2 + 1) - 1 };
  } else {
    return unsafe { unsafe_fibo((n / 2) * 2) - 1 };
  }
}

fn main() {
  let mut input = String::new();
  stdin().read_to_string(&mut input).unwrap();
  let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

  let a = input.next().unwrap();
  let b = input.next().unwrap();
  // let time_start = std::time::Instant::now();

  let sum_b = fibo(b + 2) as u128 % 10_000_000_000;
  let sum_a = fibo(a + 1) as u128 % 10_000_000_000;
  // println!("{} {}", sum_b, sum_a);
  let sum = if sum_a > sum_b {
    ((sum_b as i128 - sum_a as i128) + 10_000_000_000) as u128
  } else {
    sum_b - sum_a
  } % 1_000_000_000;
  // let elapsed = time_start.elapsed().as_nanos();
  println!("{}", sum);
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
