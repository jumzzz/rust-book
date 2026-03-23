struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}


fn main() {

    let input_email = "andengdeng@example.com";
    let input_user = "someusername123";

    let user1 = User {
        email: &input_email,
        username: &input_user,
        active: true,
        sign_in_count: 1,
    }

}
