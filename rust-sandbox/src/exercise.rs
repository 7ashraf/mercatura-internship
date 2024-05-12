
struct Person {
    name: String,
    age: u8,
    gender: String
}

impl Person {
    fn new(name: &str, age: u8, gender : &str) -> Person {
        Person {
            name: name.to_string(),
            age,
            gender: gender.to_string()
        }
    }

    fn speak(&self) {
        println!("My name is {name} and I am {age} years old", name=self.name, age=self.age);
    }

    fn enter_bar(&self) {
        if self.age >= 18 {
            println!("Welcome to the bar, {name}", name=self.name);
        } else {
            println!("Sorry, {name}, you are not allowed to enter the bar", name=self.name);
        }
    }





}

pub fn run(){
    let ashraf = Person::new("Ashraf", 22, "m");
    let aisha = Person::new("Aisha", 17, "f");

    let mut people: Vec<Person> = vec![ashraf, aisha];


    for i in 0..50{
        people.push(Person::new("Person", i, "m"));
    }

    for person in people.iter() {
        person.speak();
        person.enter_bar();
    }
}
