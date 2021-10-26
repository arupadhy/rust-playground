/*
  Slice is a primitive data type that borrows data owned by some other
  Consists of a pointer and length (no of elements after pointer)
*/

pub fn create_slice() {
  // create slice from a string
  // first index is included
  // index if not mentioned implies start/end
  let s = String::from("Arvind");
  let s_slice = &s[2..5];
  let new_slice = &s[..];
  let another_slice = &s[1..];
  let full_slice = &s[..];

  // create slice from an array (an array has fixed length)
  let arr = [1, 3, 5, 5];
  let arr_slice = &arr[..3];

  // create slice from vector 
  let mut v = vec![10,30,40,50];
  v.push(500);
  v.push(433423);

  let v_slice = &v[..5];
  
  println!("String slices s: {}, s_slice: {}, new_slice: {}, another_slice: {}, full_slice: {}", s, s_slice, new_slice, another_slice, full_slice);
   println!("Array slices arr: {:?}, arr_slice: {:?}", arr, arr_slice);
  println!("Vector slices v: {:?}, v_slice: {:?}", v, v_slice); 
}

// method example using slices

// This method takes borrows from array of 3 i8 values
pub fn takes_ref_to_array_only(param: &[i8; 3]) {
  println!("this is an array borrowed by this method {?}", param);
}

pub fn takes_ref_to_vector_only(param: &Vec<u8>) {
  println!("this is an vector having u8 values borrowed by this method {?}", param);
}

pub fn takes_ref_to_slice(param: &[i8]) {
  println!("this can be called by any slice {?}", param);
}
