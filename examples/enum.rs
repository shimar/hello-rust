#![allow(dead_code)]

enum WebEvent {
  PageLoad,
  PageUnload,
  KeyPress(char),
  Paste(String),
  Click { x: i64, y: i64 },
}

enum Number {
  Zero,
  One,
  Two,
}

enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}

// Create a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
      Self::Add => x + y,
      Self::Subtract => x - y,
    }
  }
}

fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::PageUnload => println!("page unloaded"),
    WebEvent::KeyPress(c) => println!("pressed '{}'", c),
    WebEvent::Paste(s) => println!("pasted \"{}\"", s),
    WebEvent::Click { x, y } => {
      println!("clicked at x={}, y={}", x, y);
    },
  }
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  let pasted = WebEvent::Paste("my text".to_owned());
  let clicked = WebEvent::Click { x: 20, y: 80 };
  let loaded = WebEvent::PageLoad;
  let unloaded = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(clicked);
  inspect(loaded);
  inspect(unloaded);

  println!("zero is {}", Number::Zero as i32);
  println!("one is {}", Number::One as i32);
  println!("two is {}", Number::Two as i64);

  println!("roses are #{:06x}", Color::Red as i32);
  println!("viorets are #{:06x}", Color::Blue as i32);

  let x = Operations::Add;
  println!("x is {}", x.run(1, 2));
}
