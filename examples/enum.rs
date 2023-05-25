enum WebEvent {
  PageLoad,
  PageUnload,
  KeyPress(char),
  Paste(String),
  Click { x: i64, y: i64 },
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
}
