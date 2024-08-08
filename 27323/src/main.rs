use std::io::{stdin, Read};
macro_rules! ni {
  ($ip:ident) => {
    $ip.next().unwrap()
  };
}

fn main() {
  let mut ip = String::new();
  stdin().read_to_string(&mut ip).unwrap();
  let mut ip = ip.split_ascii_whitespace().flat_map(str::parse::<usize>);
  let x = ni!(ip);
  let y = ni!(ip);
  println!("{}", x * y);
}
