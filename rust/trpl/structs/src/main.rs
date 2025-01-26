fn main() {
    let mut user = User {
        email: String::from("sinnerson@gmail.com"),
        username: String::from("sinnerson"),
        active: true,
        sign_in_count: 1
    };

    println!("User email: {email}", email = user.email);
    let email = user.email;
    
    user.email = String::from("newemail@gmail.com");
    println!("User email: {email}", email = user.email);
    println!("User email: {email}", email = email);

    let mut another_user = build_user(String::from("new_user@gmail.com"), String::from("new_user"));
    println!("Another user email: {email}", email = another_user.email);

    let modified_user_no_owner = build_user_by_reference(&mut another_user);
    println!("Modified user (no owner) email: {email}", email = modified_user_no_owner.email);
    println!("Another user (owner) email: {email}", email = another_user.email);

    let black = Color(0, 0, 0);
    println!("Black color: {0}, {1}, {2}", black.0, black.1, black.2);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color (i32, i32, i32);

// This function creates a new user object and returns it.
// The function has ownership of the user object until the end of the function.
fn build_user(email: String, username: String) -> User {
    return User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }; 
}

// This function modifies the user passed to it and returns a reference to it.
// The function has no ownership of the user object. 
fn build_user_by_reference(user: &mut User) -> &User {
    user.email = String::from("modifiedemail@gmail.com");
    user.username = String::from("modifiedusername");
    return user;
}