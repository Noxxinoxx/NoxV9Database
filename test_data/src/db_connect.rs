use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::json;
use std::io::prelude::*;
use std::net::TcpStream;
static IP_ADDRESS: &str = "localhost:3001";

//used for creating fake/zero data for DatabaseCom.
pub fn zero_data() -> Vec<HashMap<String,Vec<String>>> {
    let mut data = Vec::new();
    let mut map = HashMap::new();
    let mut inside_hashmap = Vec::new();
    let mut inside_hashmap2 = Vec::new();
    let mut map2 = HashMap::new();
    inside_hashmap2.push("fake1-data2".to_string());
    inside_hashmap2.push("fake2-data2".to_string());
    inside_hashmap.push("fake1-data".to_string());
    inside_hashmap.push("fake2-data".to_string());
    map.insert("0".to_string(),inside_hashmap);
    map2.insert("1".to_string(),inside_hashmap2);
    data.push(map);
    data.push(map2);
 
    return data;
}
//single line data send creates one line add to the database csv.
//this is usefull if you have an hole item you want to add to the database.
pub fn create_data_single_line(data:String) -> Vec<HashMap<String,Vec<String>>>{
    let mut return_data = Vec::new();

    let data = data.split(",");

    let data: Vec<String> = data.into_iter().map(|x| x.to_string()).collect();

    for i in 0..data.len(){
        let mut hashmap = HashMap::new();
        let mut inside_hashmap = Vec::new();
        
        inside_hashmap.push(data.get(i).unwrap().to_string());
        hashmap.insert(i.to_string(),inside_hashmap);
        return_data.push(hashmap);
    }

    println!("add_job database data : {:?}", return_data);

    return return_data; 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseCom {
    pub command : String,
    pub command_data : String,
    pub data: Vec<HashMap<String,Vec<String>>>,
    pub cluster_name : String
}


impl DatabaseCom {
    //make a new DatabaseCom strct
    pub fn new(command:String, command_data:String, data:Vec<HashMap<String,Vec<String>>>,cluster_name:String) -> DatabaseCom {
        DatabaseCom {
            command : command,
            command_data:command_data,
            data : data,
            cluster_name:cluster_name

        }
        }


    //sending command to send data to the database
    pub fn send_database(&self) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(IP_ADDRESS)?;

        let mut buffer = [0; 10240];


        let json = json!( {
            "command" : self.command,
            "cluster_name" : self.cluster_name,
            "data" : self.data,
            "command_data": self.command_data
        });

        let json = json.to_string();

        //stream.write("&gc:Job_Cluster.csv:".as_bytes())?;
        //stream.write("&gic:Job_Cluster.csv:&data:1:".as_bytes())?;
        stream.write(json.as_bytes())?;


        let _ = stream.read(&mut buffer);

        println!("The json obj that was send : {}", String::from_utf8_lossy(&buffer[..]));
        Ok(())

    }
    //getting command for getting data from the database
    pub fn get_database(&self) -> std::io::Result<String> {
        let mut stream = TcpStream::connect(IP_ADDRESS)?;

        let mut buffer = [0; 10240];

        let json = json!( {
            "command" : self.command,
            "cluster_name" : self.cluster_name,
            "data" : self.data,
            "command_data": self.command_data
        });

        let json = json.to_string();

        //stream.write("&gc:Job_Cluster.csv:".as_bytes())?;
        //stream.write("&gic:Job_Cluster.csv:&data:1:".as_bytes())?;
        stream.write(json.as_bytes())?;

        let _ = stream.read(&mut buffer);

        let buffer_data = String::from_utf8_lossy(&buffer[..]).to_string();

        let buffer_data = buffer_data.trim_matches(char::from(0));
        //this parse worked for database data but i dont think this will work for command data.
        let test_parse : DatabaseCom = serde_json::from_str(buffer_data)?;

        println!("test parse data : {:?}",test_parse);

        println!("data from database : {:?}", &buffer_data);

        Ok(buffer_data.to_string())
        
    }


}


