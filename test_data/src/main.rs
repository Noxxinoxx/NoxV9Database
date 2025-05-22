mod db_connect;
fn main() {



    let db_reader = db_connect::DatabaseCom::new("&gc".to_string(),"1".to_string(), db_connect::zero_data(),"Job_Status.csv".to_string());
    

    //let data = db_reader.get_database().unwrap();
    let data_send = db_reader.send_database().unwrap();
    
    //println!("{}",data);

    println!("sending data : {:?}", data_send);



}
