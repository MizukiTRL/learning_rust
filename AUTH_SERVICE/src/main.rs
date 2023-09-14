use AUTH_SERVICE::auth_utils::models::Credentials;
fn main(){

    AUTH_SERVICE::authenticate(Credentials{username: String::from("pedro"), password: String::from("pascal")});
}