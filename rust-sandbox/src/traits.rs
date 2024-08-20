struct person {
    first_name: String,
    last_name: String,
    age: u8,
}
trait HasName {
    fn first_name(&self) -> String;
    fn last_name(&self) -> String;
}

impl HasName for person {
    fn first_name(&self) -> String {
        self.first_name.clone()
    }
    fn last_name(&self) -> String {
        self.last_name.clone()
    }
}

trait HasFullName where Self: HasName {
    fn full_name(&self) -> String;
}
impl<T> HasFullName for T where T: HasName {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}
impl person {
    fn new(first_name: String, last_name: String , age: u8) -> Self {
        Self { first_name, last_name, age }
    }
}

trait Can_Walk {
    fn can_walk(&self);
}

impl Can_Walk for person {
    fn can_walk(&self){
        println!("{} can walk", self.first_name);
    }
}

fn walk<T>(person: T) 
where T: Can_Walk {
    person.can_walk();
}

pub fn run() {
    let person = person::new("John".to_string(), "Doe".to_string(), 25);
    println!("{}", person.full_name());
    person.can_walk();

}