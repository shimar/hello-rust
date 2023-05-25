#![allow(dead_code)]

#[derive(Debug)]
struct Person {
  name: String,
  age: u8
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn main() {
  let name = String::from("Peter");
  let age = 27;
  let peter = Person { name, age };

  println!("{:?}", peter);

  // Instantiate a `Point`
  let point: Point = Point { x: 10.3, y: 0.4 };

  // Access the fields of the point
  println!("point coordinates:  ({}, {})", point.x, point.y);

  // Make a new point by using struct update syntax to use the fields of our other one
  let bottom_right = Point { x: 5.2, ..point };
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  // Destructure the point using `let` binding.
  let Point { x: left_edge, y: top_edge } = point;

  let _rectangle = Rectangle {
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right: bottom_right,
  };

  let _unit = Unit;

  let pair = Pair(1, 0.1);
  println!("pair containts {:?} and {:?}", pair.0, pair.1);

  let Pair(integer, decimal) = pair;
  println!("pair containts {:?} and {:?}", integer, decimal);

  let r = Rectangle {
    top_left: Point { x: 0.0, y: 0.0 },
    bottom_right: Point { x: 10.0, y: 10.0 },
  };
  let rect_area = rect_area(r);
  println!("Rectangle area: {}", rect_area);

  let p = Point { x: 0.0, y: 0.0 };
  let l = 10.0;
  let r = square(p, l);
  println!("Square: {:?}", r);
}

fn rect_area(rect: Rectangle) -> f32 {
  let Rectangle { top_left, bottom_right } = rect;
  let Point { x: left_edge, y: top_edge } = top_left;
  let Point { x: right_edge, y: bottom_edge } = bottom_right;
  let width = right_edge - left_edge;
  let height = bottom_edge - top_edge;
  width * height
}

fn square(p: Point, l: f32) -> Rectangle {
  let Point { x, y } = p;
  Rectangle {
    top_left: Point { x, y },
    bottom_right: Point { x: x + l, y: y + l },
  }
}
