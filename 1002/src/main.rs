use std::io::{stdin, Read};

fn main() {
  let mut input = String::new();
  stdin().read_to_string(&mut input).unwrap();
  let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
  let counts = input.next().unwrap();
  for _ in 0..counts {
    let [x1, y1, r1, x2, y2, r2]: [i32; 6] = [
      input.next().unwrap(),
      input.next().unwrap(),
      input.next().unwrap(),
      input.next().unwrap(),
      input.next().unwrap(),
      input.next().unwrap(),
    ];
    println!(
      "{}",
      if x1 == x2 && y1 == y2 && r1 == r2 {
        -1
      } else {
        let distance = ((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64;
        let ra = (r1 + r2).pow(2) as f64;
        let rb = (r1 - r2).pow(2) as f64;

        if distance == ra || distance == rb {
          1
        } else if distance < ra && distance > rb {
          2
        } else {
          0
        }
      }
    );
  }
}
