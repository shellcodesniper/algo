use std::io::{stdin, Read};
macro_rules! ni {
  ($ip:ident) => {
    $ip.next().unwrap()
  };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
  x: i64,
  y: i64,
}

fn ccw(A: Point, B: Point, C: Point) -> i32 {
  let value = (B.x - A.x) * (C.y - A.y) - (B.y - A.y) * (C.x - A.x);
  if value > 0 {
    return 1;
  } else if value < 0 {
    return -1;
  } else {
    return 0;
  }
}

fn is_point_on_segment(P: Point, A: Point, B: Point) -> bool {
  let min_x = A.x.min(B.x);
  let max_x = A.x.max(B.x);
  let min_y = A.y.min(B.y);
  let max_y = A.y.max(B.y);
  P.x >= min_x && P.x <= max_x && P.y >= min_y && P.y <= max_y
}

fn do_segments_intersect(P1: Point, P2: Point, Q1: Point, Q2: Point) -> bool {
  if P1 == Q1 || P1 == Q2 || P2 == Q1 || P2 == Q2 {
    return true;
  }

  let ccw1 = ccw(P1, P2, Q1);


  if ccw1 == 0 && is_point_on_segment(Q1, P1, P2) {
    return true;
  }

  let ccw2 = ccw(P1, P2, Q2);
  if ccw2 == 0 && is_point_on_segment(Q2, P1, P2) {
    return true;
  }

  let ccw3 = ccw(Q1, Q2, P1);
  if ccw3 == 0 && is_point_on_segment(P1, Q1, Q2) {
    return true;
  }

  let ccw4 = ccw(Q1, Q2, P2);
  if ccw4 == 0 && is_point_on_segment(P2, Q1, Q2) {
    return true;
  }

  if ccw1 * ccw2 < 0 && ccw3 * ccw4 < 0 {
    return true;
  }

  return false;
}

fn main() {
  let mut ip = String::new();
  stdin().read_to_string(&mut ip).unwrap();
  let mut ip = ip.split_ascii_whitespace().flat_map(str::parse::<i64>);
  let total_count = ni!(ip) as usize;
  let mut line_list = Vec::<[Point; 2]>::with_capacity(total_count);

  for _ in 0..total_count {
    let [x1, y1, x2, y2] = [ni!(ip), ni!(ip), ni!(ip), ni!(ip)];
    line_list.push([Point { x: x1, y: y1 }, Point { x: x2, y: y2 }]);
  }

  let mut intersect_count = 0;
  for first_idx in 0..line_list.len() {
    let [P1, P2] = line_list[first_idx];
    for second_idx in (first_idx + 1)..line_list.len() {
      let [Q1, Q2] = line_list[second_idx];
      if do_segments_intersect(P1, P2, Q1, Q2) {
        intersect_count += 1;
      }
    }
  }
  println!("{}", intersect_count);
}
