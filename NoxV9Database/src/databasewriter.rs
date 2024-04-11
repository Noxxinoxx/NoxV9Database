use std::fs;

pub struct Writer {
    path : String
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            path : "D:/NoxDatabase/NoxV9Database/NoxV9Database/NoxV9Database/Database/database.txt".to_string(),
        }
    }

    pub fn write_database(data: String) -> bool {
        //create a string. 
        return true;
        
    }
    pub fn read_database(self) -> String{
        let data : String = fs::read_to_string(self.path).expect("can read file");
        println!("{}", data);   
        return data;
    }
    
}