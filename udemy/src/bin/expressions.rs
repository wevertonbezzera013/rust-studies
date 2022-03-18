enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("Can Access: {}", can_access_file);
}