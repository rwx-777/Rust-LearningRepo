//Structs - Are used to create custom data types

//Tradiotional Struct
struct colour {
    red: u8,
    green: u8,
    blue: u8
}


//Tuples Struct
struct hair_colour(u8,u8,u8);

//Examples for practice
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    //Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }

}

pub fn run() {

    let mut c = colour{
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Colour: {} {} {}", c.red,c.green, c.blue);


    let mut b = hair_colour(255,0,0);

    println!("Hair-Colour: {} {} {}", b.0, b.1, b.2);


    let mut p = Person::new("Elliot", "Alderson");
    println!("Full Name: {}",p.full_name());
    p.set_last_name("Williams");
    println!("Person: {} {}", p.first_name, p.last_name);

    println!("Person Tuple: {:?}",p.to_tuple());

}