mod db;
use db::*;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    let conn = init_database().expect("Failed to init db");
    clr();

    println!("Welcome to X vault\nSave your passwords\n");
    
    loop{
        println!("
            Password manager menu:\n
            1. Add service\n
            2. List all service\n
            3. Search using exact service name\n
            4. Quit\n
        ");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service:"),
                    prompt("Username:"),
                    prompt("Password:"),
                );
                write_password_to_db(
                    &conn,
                    &entry.service,
                    &entry.username,
                    &entry.password,
                )
                .expect("Failed to write to db");
                println!("Entry add success");
            }
            "2" => {
                clr();
                let services = read_passwords_from_db(&conn).unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service={}
                        - Username:{}
                        - Password:{}
                        ",
                        item.service, item.username, item.password
                    )
                }
            }
            "3" => {
                clr();
                let search = prompt("Search by serivce name:");
                match search_service_by_name(&conn, &search) {
                    Ok(Some(entry)) => {
                        println!(
                            "
                            Service={}
                            - Username:{}
                            - Password:{}
                            ",
                            entry.service, entry.username, entry.password
                        );
                    }
                    Ok(None) => {
                        println!("Service not found");
                    }
                    Err(err) => {
                        eprintln!("Error searching for service: {}", err);
                    }
                }
            }
            "4" => {
                clr();
                println!("Data is securely stored!");
                break;
            }
            _ => println!("Invalid choice")
        }
        println!("\n\n");
    }
}