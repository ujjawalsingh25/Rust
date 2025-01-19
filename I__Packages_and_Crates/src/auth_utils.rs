pub mod models;
    //  -->>  file and folder name should be same 
    //  -->>  Since "auth_utils" having "models" code so 
    //  -->>  "models.rs" should be inside Folder name -> "auth_utils"


pub fn login(cred: models::Credentials) {
    // try to login
    crate::database::get_user();         // get database from "root"
}