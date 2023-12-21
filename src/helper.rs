use std::{
    fmt::Debug,
    fs::File,
    io::{self, IoSlice, Read, Write},
    path::Path,
};

fn print(message: &str) -> io::Result<()> {
    print!("{message}");
    io::stdout().flush()?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Person {
    firstname: String,
    lastname: String,
    age: u8,
}

pub fn read_user_entry() -> io::Result<String> {
    let mut user_entry = String::new();
    io::stdin().read_line(&mut user_entry)?;
    Ok(user_entry)
}

pub fn read_person_information() -> io::Result<Person> {
    print("Votre nom: ")?;
    let firstname = read_user_entry()?;
    print("Votre prénom: ")?;
    let lastname = read_user_entry()?;
    print("Votre age: ")?;
    let age = match read_user_entry()?.trim().parse::<u8>() {
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        Ok(a) => a,
    };
    Ok(Person {
        firstname,
        lastname,
        age,
    })
}

pub fn normalized_person(person: &Person) -> String {
    format!(
        "nom: {}prénom: {}age: {}",
        person.firstname, person.lastname, person.age,
    )
}

fn read_file_content(filepath: &str) -> io::Result<String> {
    let mut file = File::open(Path::new(filepath))?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

pub fn find_all_person() -> io::Result<String> {
    read_file_content("text.txt")
}

pub fn put_file_content(content: &str, file: &mut File) -> io::Result<()> {
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn save_person(file_path: &str, person: &Person) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    put_file_content(&normalized_person(&person), &mut file)?;
    Ok(())
}

pub fn ask_user_by_yes_or_not(message: &str) -> bool {
    println!("{} (y/n)", message);
    let response = match read_user_entry() {
        Err(_) => "n".to_string(),
        Ok(v) => v,
    };
    response.contains("y")
}

pub enum Action {
    AddNewPerson,
    PrintAllPerson,
    Quit,
    Unknown,
}

pub fn show_menu() {
    println!("Selectionner une action");
    println!("1 - Ajouter une nouvelle personne");
    println!("2 - Imprimer les personnes");
    println!("3 - Quitter");
}

pub fn get_menu_action() -> Action {
    let response: u8 = loop {
        print("> ").unwrap();
        match read_user_entry().unwrap().trim().parse::<u8>() {
            Ok(a) => break a,
            Err(_) => {
                println!("Vous devez saisir un chiffre!!!!!");
                continue;
            }
        }
    };

    match response {
        1 => return Action::AddNewPerson,
        2 => return Action::PrintAllPerson,
        3 => return Action::Quit,
        _ => return Action::Unknown,
    };
}

pub fn add_new_person() {
    loop {
        let person = match read_person_information() {
            Err(_) => {
                println!("please give correct informations!!");
                continue;
            }
            Ok(p) => p,
        };
        match save_person("src/text.txt", &person) {
            Err(_) => panic!("error save person information"),
            Ok(_) => println!("Success to save person information"),
        }
        if !ask_user_by_yes_or_not("you want add new user ?") {
            break;
        }
    }
}

pub fn export_all_person(filename: &str, persons: &[Person]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for person in persons {
        let normalized_person = normalized_person(&person);
        let line_slice = IoSlice::new(normalized_person.as_bytes());
        file.write_vectored(&[line_slice])?;
    }
    Ok(())
}
