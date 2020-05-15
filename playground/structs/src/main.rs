struct User {
    name: String,
    age: u16,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("Nome do User"),
        age: 30, 
        active: true 
    };

    let user2 = build_active_user(String::from("Nome"), 30);

}

// =====  Field Init Shorthand ======
// If te parameter has the same name from struct field, 
// it is not necessary to inform
// ex.: below don't need to be 'name: name' and 'age: age'
fn build_active_user(name: String, age: u16) -> User {
    User {
        name,
        age,
        active: true
    }
}