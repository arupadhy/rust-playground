/*
  Concept of borrowing the ownership so that owner is still responsible for cleaning up stuff and nothing is modified 
*/

struct Person {
  name: String,
  age: u8,
}

impl Person {
  fn new(name:String, age:u8) -> Person {
    Person{name, age}
  }
}

// this function borrows person instead of owning it
// so person will not be cleaned up after this function goes out of scope
fn congratulate(person: &Person) {
  println!("Congratulations {} !!!!", person.name);
}

pub fn use_person_to_understand_ownership() {
  println!("This is an example to understand ownership/borrow without clone");
  let person = Person::new(String::from("Arvind Upadhyay"), 35);
  // pass ref to congratulate function
  congratulate(&person); 
}
