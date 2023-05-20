use std::mem;

fn analyze_slice(slice: &[i32]) {
  println!("First element of the slice: {}", slice[0]);
  println!("The slice has {} elements", slice.len());
}

fn main() {
  let xs: [i32; 5] = [1, 2, 3 , 4, 5];
  // let ys: [i32; 500] = [0; 500];

  println!("xs: {:?}", xs);
  println!("First element of xs: {}", xs[0]);
  println!("Second element of xs: {}", xs[1]);
  println!("Number of elements in array: {}", xs.len());

  // arrays are stack allocated.
  println!("Array occupies {} bytes in stack.", mem::size_of_val(&xs));

  // Arrays can be automatically borrowed as slices.
  println!("Borrow the whole array as a slice.");
  analyze_slice(&xs);

  let empty_array: [i32; 0] = [];
  assert_eq!(&empty_array, &[]);
  assert_eq!(&empty_array, &[][..]);

  for i in 0..xs.len() + 1 {
    match xs.get(i) {
      Some(xval) => println!("{}: {}", i, xval),
      None => println!("Slow down! {} is too far.", i),
    }
  }
}
