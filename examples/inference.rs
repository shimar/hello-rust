fn main() {
  // Because of the annotation, the compiler knows that `elem` has type u8.
  let elem = 5u8;

  // Creat an empty vector.
  // At this point the compiler doesn't know the exact type of `vec`,
  // it just knows that it's a vector of something (`Vec<_>`).
  let mut vec = Vec::new();

  // Insert `elem` in the vector.
  vec.push(elem);
  // Aha!
  // Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

  println!("{:?}", vec);
}
