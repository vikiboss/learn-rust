fn main() {
  println!("Hello, world!");

  let mut a = 1;
  let b = 2;
  a = a + b;

  println!("a = {}, {}", a, 2);

  println!(
      "i32 {}, i8 {}, usize {}, u8 {}, f32 {}, f64 {}, bool {}, char {}",
      std::mem::size_of::<i32>(),
      std::mem::size_of::<i8>(),
      std::mem::size_of::<usize>(),
      std::mem::size_of::<u8>(),
      std::mem::size_of::<f32>(),
      std::mem::size_of::<f64>(),
      std::mem::size_of::<bool>(),
      std::mem::size_of::<char>()
  );
}
