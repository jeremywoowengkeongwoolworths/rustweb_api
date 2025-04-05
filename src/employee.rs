use serde::Serialize; 

#[derive(Serialize)] 
pub struct Employee {
    pub name: String,
    pub age: u32,
    pub position: String,
}

impl Employee {
    pub fn new(name: String, age: u32, position: String) -> Self {
        Employee { name, age, position }
    }
}

impl Employee {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_age(&self) -> &u32 { 
        &self.age   
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
 
