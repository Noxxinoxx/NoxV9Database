
use rand::Rng;
use crate::database;
use crate::hashing::Hash;

/*
    This struct handles the verification of tokens and passwords using methods verify_token & verify_password.
    In the future user struct will be implemented so that you can have private passwords, create passwords etc.
 */
pub struct Authenticator {}

impl Authenticator {

    // This method takes in a &String and simply updates the db with it using update_database.
    fn add_token_to_db(token : &String) {
        let token_vec : Vec<String> = token.split('\n').into_iter().map(|x| x.to_string()).collect();
        database::update_database(token_vec, "tokens.csv".to_string());
    }

    // This method checks with all the tokens in the db to see if any one of them are equal to the request token
    #[allow(dead_code)] // this function is not yet implemented, and thus it created an error unless allowed
    pub fn verify_token(token : &String) -> bool{
        let db_tokens = database::get_database("tokens.csv".to_string());
        if db_tokens.contains(token) {
            return true
        }
        false
    }

    // This method generates a token which has been hashed using Hash, for more info see hash struct
    pub fn token_generator() -> String {
        let random_password = Self::random_password_generator();
        let mut hasher = Hash::new();
        hasher.gen_hash_type(&random_password);
        let token = hasher.password(&random_password);
        Self::add_token_to_db(&token);
        token
    }

    // This method is quite self-explanatory and generates a random number between 16-24 and then
    // fills an empty string with random letters from the alphabet, this is the password used in token_generator
    fn random_password_generator() -> String {
        let mut rng = rand::thread_rng();
        let mut password = String::new();
        let password_len : usize = rng.gen_range(16..24);
        let mut random_number : u8;

        for _i in 0..password_len {
            random_number = rng.gen_range(0..27) + 97;
            password.push(random_number as char);
        }
        password
    }

    // This method takes in a string from the client and checks if that string is equal to the
    // password in the hashes.csv database, NOT currently optimized for multiple users.
    pub fn verify_password(user_password : String) -> bool {
        let local_password = database::get_database("hashes.csv".to_string());
        if user_password == local_password {
            return true
        }
        false
    }
 }

