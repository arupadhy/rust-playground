// enum can hold some data as well

enum Clock {
  Sundial(u8),
  Digital(u8, u8),
  Analog(u8, u8, u8),
}

/**
  Rust push ups
**/

fn main() {
    let mut a = 12;
    let mut b = 300;
    
    let arr = [1,2,3];
    let slice = &arr[0..1];

    // tuple in rust
    let x = (1,2.42,"i am a string", 'c', 'u');
    
    println!("a is {}, b is {}, x.0 is {}, x.1 is {}, x.2 is {}, x.3 is {}", a, b, x.0, x.1, x.2, x.3);
    println!("Hello, world!");

    println!("square(2): {}", square(2));
    let y = 10;

// if expression in rust
   let if_expression_value = if y>10 {
     "y is greater than 10"
   }else {
     "y is less than 10 now"
   };
   // it is important to terminate if expression

   println!("if_expression_value is: {}", if_expression_value);

// for loop with range
   for i in 1..11 {
    println!("now serving {}", i);
   }

// while loop
   let mut i = 10;
   while i<=20 {
     println!("while loop in progress for i = {}", i);
     i += 1;
   }

// basic use of match
   let x = 1000;
   
   match x {
     1 => println!("did not match for 1"),
     2 => println!("Did not match yet for 2"),
     100 => println!("Did not match yet for 100"),
     _ => println!("Did not match with any pattern"),
   }

// usage of match expression
  let (die1, die2) = (1, 5);

  match (die1, die2) {
    (1,4) => println!("You rolled 1 and 4"),
    (_,5) | (5,_) => {
       println!("one of 5 was rolled");
       println!("match expression are g8 in rust");
    },
    _ => println!("matched nothing"),

  }

// Basic Enum

enum Position {
  Wing,
  Center,
  Defense,
}


// struct in rust

struct Player {
  name: String,
  number: u8,
  position: Position,
}

// create mutable instance of struct
let player = Player {
   name: String::from("Arvind Upadhyay"),
   number: 35,
   position: Position::Center, 
};

println!("{} is {} years old ", player.name, player.number);

// diff kinds of struct
// Tuple structs

struct Triangle(u32, u32, u32); // similar to Tuple but forces type
let triangle = Triangle(10, 35, 35);

// unit structs
struct UnitStruct;

// there are enum variants that look like struct




// Methods that belong to the struct

impl Player {
  // self is first argument for the struct
  // shoot_puck is a behaviour on Player
  // associated functions don't take self, methods do
  // notice use of &
  fn shoot_puck(&self, seconds_remaining: u16) {
    if seconds_remaining > 50 {
       println!("Keep dribbling");
       return;
    }
    println!("shoot puck will figure out some detail");
    match self.position {
      Position::Center => println!("GOAL"),
      _ => println!("Missed it"),
    }
  }

// method that will mutate the members
// Understand concept of owenership in rust
fn move_position(&mut self, seconds_remaining: u8) {
  if seconds_remaining < 10 {
     println!("Not enough time left");
     self.number -= 1;
     return;
  } else {
    self.number += 1;
    self.position = Position::Defense;
    println!("move player to defense position");
  }
}

// This is an associated function that will create a new Player (similar to new in other lang)
// usually convention in rust
  fn new(name: String, number: u8, position: Position) -> Player {
     Player {
       name: name,
       number: number,
       position: position,
     } 
  }
}
player.shoot_puck(10);

// create new Player and try shotting puck again
let new_player = Player {
    name: String::from("New player"),
    number: 50,
    position: Position::Defense,
  };

new_player.shoot_puck(80);

// create another player using associated function
  let mut third_player = Player::new(String::from("Foo bar"), 40, Position::Defense);
  third_player.shoot_puck(20);

  // calling mutating method
  third_player.move_position(20);
}

fn tell_time(clock: Clock) {
  // here match can be used along with destructuring
}

fn square(no: i32) -> i32 {
  return no * no;
}


