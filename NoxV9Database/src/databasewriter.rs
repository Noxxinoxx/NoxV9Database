use std::fs;

pub struct Writer {
    path : String
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            path : "C:/Users/Noxit/Desktop/NoxV9Database/NoxV9Database/Database/database.txt".to_string(),
        }
    }

    pub fn write_database(data: String) -> bool {
        //create a string. 
        return true;
        
    }
    pub fn read_database(&self) -> String{
        let data : String = fs::read_to_string(&self.path).expect("can read file");  
        return data;
    }

    pub fn read_database_id(&self,id: i32) -> String{
        
        let data : String = self.read_database();
        let indexable : Vec<&str> = data.split(";").collect();
        
        return indexable[id as usize].to_string();
    
    }
    
}