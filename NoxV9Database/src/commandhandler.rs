use crate::database;
use crate::tools;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*
{
    "command" : "nsj",
    "data" : [
        {"unity": ["data1", "data2", "data3", "data4"]}

    ],
    "auth" : "token"
}
*/
#[derive(Serialize,Deserialize,Debug)]
struct Status {
    status_message:String
}
impl Status {
    
    pub fn new_status(status:&String) -> Status{
        
        Status{
            status_message:status.to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    command: String,
    cluster_name: String,
    data: Vec<HashMap<String, Vec<String>>>,
    command_data: String,
}

impl Request {
    //Format: &cc:cluster_name:&data:1,1,1,1,2,55,hej:
    //this handle an imcomming requests data and puts it in a Request type.
    pub fn new_request(request: &String) -> Request {
        let req = request.clone();
        let name_data: (String, Vec<String>, String) = tools::command_data_combo(&request);
        let name: String = name_data.0;
        let data: Vec<String> = name_data.1;
        let command: String = name_data.2;
        let index = tools::get_index_cluster(&data);
        let mut data_vec = Vec::new();

        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        map.insert("data".to_string(), data);
        data_vec.push(map);

        Request {
            command: command,
            cluster_name: name,
            data: data_vec,
            command_data: "Command".to_string(),
        }
    }

    //this converts data from the database into a Request type that we can send out in json format.
    pub fn new_database(database_data: &String) -> Request {
        //the length of data is how many lines we have.
        let data = database_data.split("\n");
        let mut data: Vec<&str> = data.collect();

        let info_length = data[0].split(",");
        let mut info_length: Vec<&str> = info_length.collect();
        info_length.pop();
        let mut data_vec = Vec::new();

        for col in 0..info_length.len() {
            let col: i32 = col as i32;
            let mut col_hashmap: HashMap<String, Vec<String>> = HashMap::new();
            let mut col_vec: Vec<String> = Vec::new();
            let mut diff = 1;
            if (data.len() == 1) {
                diff = 0
            }
            for row in 0..(data.len() - diff) {
                let row: i32 = row as i32;
                &col_vec.push(tools::get_col_data_points(&database_data, &row, &col));
            }
            &col_hashmap.insert(col.to_string(), col_vec);
            &data_vec.push(col_hashmap);
        }

        Request {
            command: "database_data".to_string(),
            cluster_name: "cluster".to_string(),
            data: data_vec,
            command_data: "Command".to_string(),
        }
    }

    pub fn json_to_database_string_format(&self) -> String {
        let mut builder: String = "".to_string();

        let data = &self.data;

        let data_col: &Vec<String> = data.get(1).unwrap().get(&1.to_string()).unwrap();

        for index in 0..data_col.len() {
            for data_index in 0..data.len() {
                let mut line_data = data
                    .get(data_index)
                    .unwrap()
                    .get(&data_index.to_string())
                    .unwrap()
                    .get(index)
                    .unwrap()
                    .to_owned();
                //println!("{:?}", line_data);
                &line_data.push_str(",");

                builder.push_str(&line_data);
            }
            builder.push_str(&"\n");
        }

        return builder;
    }
}

pub fn command_handler(request: String) -> String {
    let req = request.clone();

    let res = req.trim_matches(char::from(0));

    //let req_as_struct : serde_json::Value = serde_json::to_value(&res).unwrap();
    //println!("{:#?}", req_as_struct);

    let req_as_struct: Request = serde_json::from_str(&res).unwrap();
    let req_data_as_string: String = req_as_struct.json_to_database_string_format();

    let command = req_as_struct.command;
    let command_data: String = req_as_struct.command_data;
    let index: i32 = tools::convert_data_to_index(&command_data);
    let data = req_as_struct.data;
    let cluster = req_as_struct.cluster_name;
    //commands here are for database interactions like getting data from a cluster.
    let return_data = match command.as_str() {
        "&gc" => database::get_database(&cluster),
        "&gic" => database::get_index_database(&cluster, &index),
        _ => "false".to_string(),
    };
    let mut d = Request::new_database(&return_data);

    //and command here is for get info or configuring things about the cluster.
    if return_data == "false" {
        let return_data: String = match command.as_str() {
            "&cc" => database::new_custom_object(&req_data_as_string, &cluster),
            "&ac" => database::update_database(&req_data_as_string, &cluster),
            "&rc" => database::clear_database(&cluster),
            "&udjf" => command_unity_done_with_job_false(),
            "&udjt" => command_unity_done_with_job_true(),
            "&udjget" => command_unity_done_with_job_get(),
            "&ubp" => command_unity_button_pressed(&command_data),
            "&nrbp" => command_test_tool_return_button_press(&command_data),
            "&nsj" => command_test_tool_stop_jobs(),
            _ => "not a server command".to_string(),
        };
        let status = Status::new_status  (&return_data);
        serde_json::to_string(&status).unwrap()

    }else {
        d.json_to_database_string_format();
        serde_json::to_string(&d).unwrap()
    }

}

fn command_unity_done_with_job_true() -> String {
    database::clear_database(&"Unity_Done_With_Job.csv".to_string());

    let data = "Job,\ntrue,\n".to_string();
    let database = database::update_database(&data, &"Unity_Done_With_Job.csv".to_string());

    return "Unity job true,".to_string();
}

fn command_unity_done_with_job_false() -> String {
    database::clear_database(&"Unity_Done_With_Job.csv".to_string());

    let data = "Job,\nfalse,\n".to_string();
    let database = database::update_database(&data, &"Unity_Done_With_Job.csv".to_string());

    return "Unity job false,".to_string();
}

fn command_unity_done_with_job_get() -> String {
    let index_db_index: i32 = 1;

    let co: String =
        database::get_index_database(&"Unity_Done_With_Job.csv".to_string(), &index_db_index);
    let mut builder: String = "&ud".to_owned();
    builder.push_str(&co);

    return builder.to_string();
}

fn command_unity_button_pressed(req: &String) -> String {
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
    let mut build: String = "Start,Stop,Brand,Service,\n".to_string();

    as_vec[index as usize] = "true".to_string();

    //as_vec.pop();

    let as_vec: String = as_vec.join(",");

    build.push_str(&as_vec);

    let database = database::update_database(&build, &"Positions_Buttons.csv".to_string());

    let res = "Success, Button is pressed".to_string();
    return res;
}

fn command_test_tool_return_button_press(req: &String) -> String {
    let database_data = database::get_index_database(&"Positions_Buttons.csv".to_string(), &1);
    let mut as_vec: Vec<String> = tools::handel_data_from_command(database_data);

    let index = tools::button_press_handel(req.clone().to_string());

    database::clear_database(&"Positions_Buttons.csv".to_string());

    let mut build: String = "Start,Stop,Brand,Service,\n".to_string();

    as_vec[index as usize] = "false".to_string();

    as_vec.pop();

    let as_vec: String = as_vec.join(",");

    build.push_str(&as_vec);

    let database = database::update_database(&build, &"Positions_Buttons.csv".to_string());

    let res = "Success, Button is pressed".to_string();
    return res;
}

fn command_test_tool_stop_jobs() -> String {
    let data = database::get_index_database(&"Job_Status.csv".to_string(), &1);
    let data = data.split(",");
    let mut data: Vec<String> = data.into_iter().map(|x| x.to_string()).collect();
    let _ = database::clear_database(&"Job_Status.csv".to_string());

    println!("data before : {:?}", data);
    data[1] = "true".to_string();
    data.pop();
    let database_data = data.join(",");

    let mut vec: String = "Unity,Test_Tool,\n".to_string();

    vec.push_str(&database_data);

    println!("data when stopped: {:?}", vec);
    let database = database::update_database(&vec, &"Job_Status.csv".to_string());

    return "all jobs are stoped!".to_string();
}
