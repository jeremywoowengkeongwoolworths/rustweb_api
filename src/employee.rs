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
 