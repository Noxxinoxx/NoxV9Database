use core::fmt;
use std::collections::HashMap;

use crate::database;
use crate::tools;

/*
{
    "command" : "nsj",
    "data" : [
        {"unity": ["data1", "data2", "data3", "data4"]}    
    
    ],
    "auth" : "token"
}
*/
struct Request {
    command : String,
    data : Vec<HashMap<String, Vec<String>>>
}

impl Request {
    //Format: &cc:cluster_name:&data:1,1,1,1,2,55,hej:
    pub fn new_request(request : &String) -> Request {
        let req = request.clone();
        let name_data: (String, Vec<String>, String) = tools::command_data_combo(&request);
        let name: String = name_data.0;
        let data: Vec<String> = name_data.1;
        let command: String = name_data.2;
        let index = tools::get_index_cluster(&data);
        let mut data_vec = Vec::new();


        let mut map : HashMap<String, Vec<String>> = HashMap::new();
        map.insert("data".to_string(), data);
        data_vec.push(map);

        Request {
            command : command,
            data : data_vec
        }
    }

    pub fn new_database(database_data : &String) -> Request{


        //the length of data is how many lines we have.
        let data = database_data.split("\n");
        let data: Vec<&str> = data.collect();
        
        let info_length = data[0].split(",");
        let info_length : Vec<&str> = info_length.collect();


        let data_as_string = data.join("");

        for row in 0..info_length.len() {
            let row : i32 = row as i32;
            for col in 0..data.len() {
                let col : i32 = col as i32;
                println!("{:?}\n", tools::get_col_data_points(&data_as_string, &row, &col));
            }
            
        }

    

        


        let mut data_vec = Vec::new();
        let mut map : HashMap<String, Vec<String>> = HashMap::new();
        data_vec.push(map);
        Request {
            command : "database_data".to_string(),
            data : data_vec
        }
    }


}

pub fn command_handler(request: String) -> String {

    



    let req = request.clone();
    let name_data: (String, Vec<String>, String) = tools::command_data_combo(&request);
    let name: String = name_data.0;
    let data: Vec<String> = name_data.1;
    let command: String = name_data.2;
    let index = tools::get_index_cluster(&data);
    

    let return_data: String = match command.as_str() {
        "&cc" => database::new_custom_object(&data, &name),
        "&ac" => database::update_database(&data, &name),
        "&gc" => database::get_database(&name),
        "&gic" => database::get_index_database(&name, &index),
        "&rc" => database::clear_database(&name),
        "&udjf" => command_unity_done_with_job_false(),
        "&udjt" => command_unity_done_with_job_true(),
        "&udjget" => command_unity_done_with_job_get(&name, &data),
        "&ubp" => command_unity_button_pressed(&req),
        "&nrbp" => command_test_tool_return_button_press(&req),
        "&nsj" => command_test_tool_stop_jobs(),
        _ => "not a server command".to_string(),
    };


    let req : Request = Request::new_database(&return_data);

    return return_data;
}

fn command_unity_done_with_job_true() -> String {
    database::clear_database(&"Unity_Done_With_Job.csv".to_string());
    let mut vec: Vec<String> = Vec::new();
    vec.push("Job".to_string());
    let database = database::update_database(&vec, &"Unity_Done_With_Job.csv".to_string());

    let mut vec: Vec<String> = Vec::new();
    vec.push("true".to_string());
    let database = database::update_database(&vec, &"Unity_Done_With_Job.csv".to_string());

    return "Unity job true".to_string();
}

fn command_unity_done_with_job_false() -> String{
    database::clear_database(&"Unity_Done_With_Job.csv".to_string());
    let mut vec: Vec<String> = Vec::new();
    vec.push("Job".to_string());
    let database = database::update_database(&vec, &"Unity_Done_With_Job.csv".to_string());

    let mut vec: Vec<String> = Vec::new();
    vec.push("false".to_string());
    let database = database::update_database(&vec, &"Unity_Done_With_Job.csv".to_string());

    return "Unity job false".to_string();
}

fn command_unity_done_with_job_get(name: &String, data: &Vec<String>) -> String{
    let index_db_index: i32 = tools::get_index_cluster(data);

    let co = database::get_index_database(&name, &index_db_index);
    let mut builder: String = "&ud".to_owned();
    builder.push_str(&co);

    return builder.to_string();
}

fn command_unity_button_pressed(req: &String) -> String{
    let database_data = database::get_index_database(&"Positions_Buttons.csv".to_string(), &1);
    let mut as_vec: Vec<String> = tools::handel_data_from_command(database_data);

    let index = tools::button_press_handel(req.clone().to_string());

    println!("button index is : {}", index);
    //checker so you cant spam the button.
    if (as_vec.iter().any(|x| x == "true")) {
        println!("Noa Test Tool Working on the job!");
        let res = "A button press is allready queued!".to_string();
        
        return res;
    }

    database::clear_database(&"Positions_Buttons.csv".to_string());
    let mut vec: Vec<String> = Vec::new();
    vec.push("Start,Stop,Brand,Service,".to_string());
    let database = database::update_database(&vec, &"Positions_Buttons.csv".to_string());

    as_vec[index as usize] = "true".to_string();

    as_vec.pop();

    let database = database::update_database(&as_vec, &"Positions_Buttons.csv".to_string());

    let res = "Success, Button is pressed".to_string();
    return res;

}

fn command_test_tool_return_button_press(req: &String) -> String{
    let database_data = database::get_index_database(&"Positions_Buttons.csv".to_string(), &1);
    let mut as_vec: Vec<String> = tools::handel_data_from_command(database_data);

    let index = tools::button_press_handel(req.clone().to_string());

    database::clear_database(&"Positions_Buttons.csv".to_string());
    let mut vec: Vec<String> = Vec::new();
    vec.push("Start,Stop,Brand,Service,".to_string());
    let database = database::update_database(&vec, &"Positions_Buttons.csv".to_string());

    as_vec[index as usize] = "false".to_string();

    as_vec.pop();

    println!("{:?}", as_vec);

    let database = database::update_database(&as_vec, &"Positions_Buttons.csv".to_string());

    let res = "Success, Button is pressed".to_string();
    return res;

}

fn command_test_tool_stop_jobs() -> String{

    
    let data = database::get_index_database(&"Job_Status.csv".to_string(), &1);
    let data = data.split(",");
    let mut data: Vec<String> = data.into_iter().map(|x| x.to_string()).collect();
    let _ = database::clear_database(&"Job_Status.csv".to_string());

    println!("data before : {:?}", data);
    data[1] = "true".to_string();
    data.pop();
    let database_data = data.join(",");
    
    let mut vec: Vec<String> = Vec::new();
    vec.push("Unity,Test_Tool".to_string());
    let database = database::update_database(&vec,&"Job_Status.csv".to_string());
    
    let mut headers: Vec<String> = Vec::new();
    headers.push(database_data);
    println!("data when stopped: {:?}", headers);
    let database = database::update_database(&headers,&"Job_Status.csv".to_string());


    return "all jobs are stoped!".to_string();
}