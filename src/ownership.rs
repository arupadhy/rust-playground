pub fn say(input: String) {
  println!("say() called with {}", input);
}

pub fn change_ownership(input: String) {
  let new_owner = input;
  println!("change_ownership called on {}", new_owner);
}

pub fn two() -> String {
  println!("two() called and will return ownership of string");
  return String::from("Hello");
}

pub fn pluralize(mut input: String) -> String {
  input.push_str("s");
  return input;
}
