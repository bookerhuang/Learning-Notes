fn foo(s: &[u32]) {
}

fn main() {
  let v: Vec<u32> = vec![1,2,3,4,5];
  foo(&v);
  let a_slice = v.as_slice();
  foo(a_slice);
}