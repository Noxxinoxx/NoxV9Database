/*
    Command handeler for tcp res data.
    This function takes in a String type witch is the tcp res data
    and return a Vec with the split tcp command data.
    A command can look like this.
    Exemple: &cc:cluster_name:&data:data1,data2,data3:
    This command will take create a cluster with name cluster_name and add the data : data1, data2, data3.
*/
pub fn handle_command(req_data: String) -> Vec<String> {
    let cluster_name = req_data.split(":");
    let data: Vec<String> = cluster_name.into_iter().map(|x| x.to_string()).collect();
    return data;
}

/*
    This function will handle the data that we get from a tcp res.
    so the data1, data2, data3.
    It will split the data and add it to a Vec<String> and then return it.
*/
pub fn handel_data_from_command(data: String) -> Vec<String> {
    let spliter = data.split(",");
    let final_data: Vec<String> = spliter.into_iter().map(|x| x.to_string()).collect();

    return final_data;
}

pub fn button_press_handel(clone_req: String) -> i32 {
    //let data = handle_command(clone_req).clone();
    //let button: String = data.get(1).unwrap().to_string();

    let index: i32 = match clone_req.as_str() {
        "Start" => 0,
        "Stop" => 1,
        "Brand" => 2,
        "Service" => 3,
        _ => panic!(),
    };

    return index;
}

pub fn get_index_cluster(data: &Vec<String>) -> i32 {
    let data = data.get(0).unwrap_or(&"0".to_string()).parse();
    if (data.is_ok()) {
        return data.unwrap_or(0);
    }
    return 0;
}

pub fn convert_data_to_index(command_data: &String) -> i32 {
    let data = command_data.parse();
    if data.is_ok() {
        return data.unwrap();
    }
    return -1;
}

pub fn get_col_data_points(data: &String, row: &i32, col: &i32) -> String {
    let data = get_row_data_points(data, row);
    let indexable: Vec<&str> = data.split(",").collect();

    let col: usize = *col as usize;
    return indexable[col].to_string();
}
pub fn get_row_data_points(data : &String, row: &i32) -> String {


    let indexable: Vec<&str> = data.split("\n").collect();
    let row: usize = *row as usize;

    return indexable[row].to_string();
}

/*





*/

pub fn command_data_combo(req: &String) -> (String, Vec<String>, String) {
    let clone_req = req.clone().to_string();
    let data = handle_command(clone_req).clone();
    let name: String = data.get(1).unwrap().to_string();
    let command: String = data.get(0).unwrap().to_string();
    let db_data: String = data.get(3).unwrap_or(&"None".to_string()).to_string();
    let co_data = handel_data_from_command(db_data);

    return (name, co_data, command);
}
