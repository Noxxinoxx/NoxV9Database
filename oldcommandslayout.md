 /*
        Command: &cc,
        Description: the cc command stands for create cluster this is called when we want to create a new cluster in the database.
        Format: &cc:cluster_name:&data:1,1,1,1,2,55,hej:
    
    if(req.contains("&cc")) {
        
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();
        //dbg!(data);

        let db_data : String = data.get(3).unwrap().to_string();
        let co_data = handel_data_from_command(db_data);

    
        let co : () = database::new_custom_object(co_data, name);

        let res = "Success, you created a cluster and wrote the struture to it.".as_bytes();
        stream.write(res).expect("Failed to write response!");

    }
    /*
        Command: &ac,
        Description: the ac command stands for add cluster this is called when we want to add data to an allready existring cluster in the database.
        Format: &ac:cluster_name:&data:1,1,1,1,2,55,hej:
    */
    else if(req.contains("&ac")) {

        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();

        let db_data : String = data.get(3).unwrap().to_string();
        let co_data = handel_data_from_command(db_data);

    
        let co = database::update_database(co_data, name);

        let res = "Success, you wrote data to the cluster".as_bytes();
        stream.write(res).expect("Failed to write response!");
    }
    /*
        Command: &gc,
        Description: the gc command stands for get cluster this is called when we want to get all the data from a cluster in the database.
        Format: &gc:cluster_name:
    */
    else if(req.contains("&gc")) {
        //&gc:database1234.csv:
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();
    
        let co = database::get_database(name);
        println!("{}", &co.as_str());
        let res = co.as_bytes();
        stream.write(res).expect("Failed to write response!");
    }
    /*
        Command: &gic,
        Description: the gic command stands for get index cluster this is called when we want to get data from an index line in a cluster.
        Format: &gic:cluster_name:&data:index_as_int:
    */
    else if(req.contains("&gic")) {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();

        let db_data : String = data.get(3).unwrap().to_string();
        let co_data = handel_data_from_command(db_data);

        let index_db_index : i32 = co_data.get(0).unwrap().to_string().parse().unwrap();
    
        let co = database::get_index_database(name, index_db_index);
        println!("{}", &co.as_str());
        let res = co.as_bytes();
        stream.write(res).expect("Failed to write response!");
    }
    /*
        Command: &rc,
        Description: the rc command stands for reset cluster this is called when we want reset/clear a cluster from all its data.
        Format: &rc:cluster_name:
    */
    else if(req.contains("&rc")) {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();

        let co = database::clear_database(name);
        println!("{}", &co.as_str());
        let res = co.as_bytes();
        stream.write(res).expect("Failed to write response!");

    }
    /*
        Command: &udjt,
        Description: the udjt command stands for unity done job true and it will change a varible in the Unity_Done_With_Job.csv cluster gets called by unity when a job is done.
        Format: &udjt:
    */
    else if(req.contains("&udjt")) {
        println!("you allways get here");
        database::clear_database("Unity_Done_With_Job.csv".to_string());
        let mut vec : Vec<String> = Vec::new();
        vec.push("Job".to_string());
        let database = database::update_database(vec,"Unity_Done_With_Job.csv".to_string());
        
        let mut vec : Vec<String> = Vec::new();
        vec.push("true".to_string());
        let database = database::update_database(vec,"Unity_Done_With_Job.csv".to_string());
        
        
        let res = "Success!".as_bytes();
        stream.write(res).expect("Failed to write response!");
    }
    /*
        Command: &udjf,
        Description: the udjf command stands for unity done job false and it will change a varible in the Unity_Done_With_Job.csv cluster gets called by Noa test tool when a new job is started.
        Format: &udjf:
    */
    else if(req.contains("&udjf")) {
        println!("unity database should be false here!");
        database::clear_database("Unity_Done_With_Job.csv".to_string());
        let mut vec : Vec<String> = Vec::new();
        vec.push("Job".to_string());
        let database = database::update_database(vec,"Unity_Done_With_Job.csv".to_string());
        
        let mut vec : Vec<String> = Vec::new();
        vec.push("false".to_string());
        let database = database::update_database(vec,"Unity_Done_With_Job.csv".to_string());
        
        
        let res = "Success!".as_bytes();
        stream.write(res).expect("Failed to write response!");
    }
    /*
        Command: &udjget,
        Description: the udjget is a get command and will send the current varible in the Unity_Done_With_Job.csv cluster.
        Format: &udjget:
    */
    else if(req.contains("&udjget")) {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();

        let db_data : String = data.get(3).unwrap().to_string();
        let co_data = handel_data_from_command(db_data);

        let index_db_index : i32 = co_data.get(0).unwrap().to_string().parse().unwrap();
    
        let co = database::get_index_database(name, index_db_index);
        let mut builder : String = "&ud".to_owned();
        builder.push_str(&co);
        println!("{}", &builder.as_str());
        let res = builder.as_bytes();
        stream.write(res).expect("Failed to write response!");
    }

    /*
        Command: &ubp,
        Description : the ubp is a command that changes a database so the test tool can launch the corresponding button VR command.
        Format: &ubp:Button:
        Button: Start,Stop,Brand,Service
    */

    else if(req.contains("&ubp")) {

        let database_data = database::get_index_database("Positions_Buttons.csv".to_string(), 1);
        let mut as_vec : Vec<String> = handel_data_from_command(database_data);

        let index = button_press_handel(req.clone().to_string());

        println!("button index is : {}", index);
        //checker so you cant spam the button.
        if(as_vec.iter().any(|x|x=="true")) {
            println!("Noa Test Tool Working on the job!");
            let res = "A button press is allready queued!".as_bytes();
            stream.write(res).expect("Failed to write response!");
            return;
        }

        database::clear_database("Positions_Buttons.csv".to_string());
        let mut vec : Vec<String> = Vec::new();
        vec.push("Start,Stop,Brand,Service,".to_string());
        let database = database::update_database(vec,"Positions_Buttons.csv".to_string());
        
        as_vec[index as usize] = "true".to_string();
        
        as_vec.pop();
        
        println!("{:?}", as_vec);

        let database = database::update_database(as_vec,"Positions_Buttons.csv".to_string());

        let res = "Success, Button is pressed".as_bytes();
        stream.write(res).expect("Failed to write response!");


    }
    /*
        Command: &nrbp,
        Description : the ubp is a command that changes a database so the test tool can launch the corresponding button VR command.
        Format: &nrbp:Button:
        Button: Start,Stop,Brand,Service
        Stands For: Noa test tool return button press.
    */
    else if(req.contains("&nrbp")) {
        let database_data = database::get_index_database("Positions_Buttons.csv".to_string(), 1);
        let mut as_vec : Vec<String> = handel_data_from_command(database_data);

        let index = button_press_handel(req.clone().to_string());

        database::clear_database("Positions_Buttons.csv".to_string());
        let mut vec : Vec<String> = Vec::new();
        vec.push("Start,Stop,Brand,Service,".to_string());
        let database = database::update_database(vec,"Positions_Buttons.csv".to_string());
        
        as_vec[index as usize] = "false".to_string();
        
        as_vec.pop();
        
        println!("{:?}", as_vec);

        let database = database::update_database(as_vec,"Positions_Buttons.csv".to_string());

        let res = "Success, Button is pressed".as_bytes();
        stream.write(res).expect("Failed to write response!");

    }
    */