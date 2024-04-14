use std::fs;
use std::fs::File;
use std::io::{Read, Write};
pub struct Writer {
    path : String
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            path : "D:/NoxDatabase/NoxV9Database/NoxV9Database/NoxV9Database/Database/database.txt".to_string(),
        }
    }

    pub fn write_database(&self, mut data: String) -> std::io::Result<()> {
        //create a string. 
        let mut database : String = self.read_database();
        data.push_str("\n");
        database.push_str(&data);

        println!("{}",database);

        let res : std::io::Result<()> = self.write_to_file(database);

        return res;
        
    }

    fn write_to_file(&self, data: String) ->  std::io::Result<()> {
        let mut file = File::create(&self.path)?;
        file.write(data.as_bytes())?;
        println!("file is written!");
        Ok(())
    
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