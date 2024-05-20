use crate::database;
use crate::tools;
enum command {
    cc,
    ac,
    gc,
    gic,
    rc,
    udjt,
    udjf,
    udjget,
    ubp,
    nrbp,
}



pub fn command_handler(request : Cow<str>) {

    
    
    
    let name_data: (String, Vec<String>, String) = tools::command_data_combo(request);
    let name : String = name_data.0;
    let data: Vec<String> = name_data.1;
    let command: String = name_data.2;


    let return_data = match command {
        command::cc => database::new_custom_object(data, name),
        command::ac => database::update_database(data, name),
        command::gc => database::get_database(name),
        command::gic => database::get_index_database(name, get_index_cluster(data)),
        command::rc => database::clear_database(name),
        _ => println!("not a server command : {}", command)
    };

    println!("{:?}"return_data);
    //return type so we know what to send to the server;


    return return_data;
}


fn get_index_cluster(data : ()) {
    return data.get(0).unwrap().to_string().parse().unwrap();
}
