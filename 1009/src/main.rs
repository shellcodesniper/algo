use std::io::{stdin, Read};

fn main() {
  let mut input = String::new();
  stdin().read_to_string(&mut input).unwrap();
  let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

  let counts = input.next().unwrap();
  for _ in 0..counts {
    let base = input.next().unwrap() % 10;
    let exp = input.next().unwrap();
    
    let result = {
      let mut temp = base;
      for _ in 1..exp {
        temp = (temp * base) % 10;
        // NOTE: 1의 자리만 남겨두면됨
      }
      temp
    };
    if result % 10 == 0 {
      println!("10");
      continue;
    } else {
      println!("{}", result % 10);
    }
  }
}
