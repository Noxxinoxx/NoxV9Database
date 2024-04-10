
use rand::prelude::*;


/*
    If you want to use a hashtype from an allready existing password then you have to 
    extract the hashtype from the hash that is saved in the database.
    when you have the hashtype then you can cross check the password.

    how this works.
    This works like a rotating hash and salt placer.
    you will give this a password t.ex. NoxPassword. if its not in the database then I will generate a new salt.
    With the new salt I will hash it / encrypt it and after that is done I will add both the salt and the hashed password
    together. If you then want to see if a user has enterd the correct password on the application you will have to retrieve the hash_type aka salt
    and then cross check the given password with the salt to see if you have a match. if you have a match the password was correct if it was woring
    then the user inputed the worong password.

    retrieve the salt.
    the salt is stored in the passwordhash in the database. It's place between every char in the password for exemple NoxPassword hashed is 3kf3412trew
    so the final hash will be 3saltksaltfsalt etc.
*/
pub struct Hash {
    hashtype: String,
    hash: String
}


impl Hash {
    
    pub fn new() -> Hash {
        Hash {
            hashtype : "".to_string(),
            hash: "".to_string()
        }
    }


    pub fn password(&mut self,password: &String, register: bool, hash_type: String) -> String{

        if(register) {
            self.hashtype = self.gen_hash_type(password);
        }else {
            self.hashtype = hash_type; 
        }
        
        let password_hash : String = self.encrypt(password);
        let mut final_password_hash: String = "".to_string();
        let mut i = 0;
        while(password_hash.len() != i && self.hashtype.len() != i) {

            let b: u8 = password_hash.as_bytes()[i as usize];
            final_password_hash += &(b as char).to_string();
            let c: u8 = self.hashtype.as_bytes()[i as usize];
            final_password_hash += &(c as char).to_string();

            i += 1;

        }
        return final_password_hash;
    }

    

    fn gen_hash_type(&self, password: &String) -> String {

        let mut random = rand::thread_rng();
        let mut hash_type : String = "".to_string();

        for i in 0..password.len() {
            let number : i32 = random.gen_range(0..9);
            hash_type += &number.to_string();
        }
        //hash_type.parse::<i32>().unwrap();
        return hash_type;

    }

    fn encrypt(&self,password: &String) -> String{

        let mut final_hash : String = "".to_string();

        //from hash and then multiply by the position.
        //34
        let alh : String = "abcdefghijklmnopqrstyvw123456789!?".to_string();

        //move the char by its hash type.
        for i in 0..(password.len() ) as i32 {
            
            let c: String = self.hashtype.as_bytes()[i as usize].to_string();
            let hash_type_number = c.parse::<i32>().unwrap();
            let mut get_placeholader = i + hash_type_number;
            
            get_placeholader = self.rotate_placeholder(get_placeholader);

            let b: u8 = alh.as_bytes()[get_placeholader as usize];
            final_hash += &(b as char).to_string();
        }

        return final_hash;

    }

    fn rotate_placeholder(&self, mut get_placeholader: i32) -> i32{
        
        if(get_placeholader < 34) {
            return get_placeholader;
        }
        
        let diff : i32 = get_placeholader - 34;
        get_placeholader = diff;
        
        return self.rotate_placeholder(get_placeholader);

    }

    pub fn retrieve_salt(&self,hashed_password: &String) -> String {
        //hash is first
        let mut salt : String = "".to_string();
        let mut i = 1;
        while(i < hashed_password.len()) {

            let b: u8 = hashed_password.as_bytes()[i as usize];
            salt += &(b as char).to_string();
            i += 2;
        }

        return salt;

    }

}