// read as: slice of i32
fn sum(values: &[i32]) -> i32 {
  let mut res = 0;
  for i in 0..values.len() {
    res += values[i]
  }
  res
}

fn debug_print() {
  let ints = [1, 2, 3];
  let floats = [1.1, 2.1, 3.1];
  let strings = ["hello", "world"];
  let ints_ints = [[1, 2], [10, 20]];
  println!("ints {:?}", ints);
  println!("floats {:?}", floats);
  println!("strings {:?}", strings);
  println!("ints_ints {:?}", ints_ints);
}

pub fn arrays_mod() {
  let arr = [10, 20, 30, 40];
  // look at that &
  let res = sum(&arr);
  println!("===========================");
  println!("From the Arrays Module");
  println!("sum {}", res);
  debug_print();
  println!("===========================");
}