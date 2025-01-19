use I__Packages_and_Crates::authenticate;
use I__Packages_and_Crates::auth_utils::models::Credentials;

fn main() {
    let cred = Credentials {
        username: String::from("ujjawal025"),
        password: String::from("admin123")
    };
    authenticate(cred);
}


// cargo install cargo-modules      -->> install "cargo-module" to see file str
// cargo modules structure --lib     -->> shows the module structure of the project