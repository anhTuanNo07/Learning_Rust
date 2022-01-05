struct Member {
    username: String,
    email: String,
    age: u64,
    active: bool,
}

fn main() {
    let mut member1 = Member {
        email: String::from("anhtuan@gmail.com"),
        username: String::from("anhtuan"),
        age: 28,
        active: true,
    };

    let member2 = create_new_member(String::from("DaoAnhTuan"), String::from("anhtuanpcipho2@gmail.com"), 30);
}

fn create_new_member (username: String, email: String, age: u64) -> Member {
    Member {
        email: email,
        username: username,
        age: age,
        active: true,
    }
}