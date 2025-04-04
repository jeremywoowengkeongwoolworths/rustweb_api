use serde::Serialize; 

#[derive(Serialize)] 
pub struct Employee {
    pub name: String,
    pub age: u32,
    pub position: String,
}
