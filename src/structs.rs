// used to create custom data types

//Traditional struct uses braces
struct Color{
    red: u8,
    green: u8,
    blue: u8
}

//Tuple struct uses parenth
struct Color2(u8,u8,u8);

//create a person struct
struct Person{
    first_name: String,
    last_name: String
}

//implement is like a constructor
impl Person{
    //construct person
    fn new(first: &str, last: &str)-> Person{
        Person{
            first_name: first.to_string(), //convert to dynamic str
            last_name: last.to_string() //convert to dynamic str
        }
    }
    //set full name
    fn full_name(&self) -> String{

        //format data to a string and return
        format!("{} {}", self.first_name, self.last_name) //remember ret no semicolon
    }
    //set last name
    fn set_lastname(&mut self, last: &str){
        
        self.last_name = last.to_string();
    }

    //Name to tuple
    fn to_tuple(self) -> (String,String){

        //return the tuple
        (self.first_name, self.last_name)
    }

}

pub fn run(){

    //create color
    let mut c = Color{
        red:255,
        green:0,
        blue:0
    };

    //change members
    c.red = 200;
    println!("Color: {} {} {}",c.red,c.green,c.blue);

    //create new color
    let mut c2 = Color2(255,0,0);
    c2.0=200;
    println!("Color: {} {} {}",c2.0,c2.1,c2.2);

    //create a new person
    let p = Person::new("John", "Doe");
    println!("Person: {} {}",p.first_name,p.last_name);
    println!("Person: {}",p.full_name());

    //change last name
    let mut m = Person::new("Mary","Doe");
    println!("Person: {}",m.full_name());
    m.set_lastname("Williams");
    println!("Person: {}",m.full_name());
    println!("Person Tuple: {:?}",m.to_tuple());



    

}