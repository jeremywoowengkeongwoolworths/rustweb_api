use serde::Serialize; 

#[derive(Serialize)] 
pub struct Car {
    pub name: String,
    pub age: u32,
}
