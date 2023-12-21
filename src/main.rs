pub mod helper;

fn main() {
    helper::show_menu();
    loop {
        match helper::get_menu_action() {
            helper::Action::AddNewPerson => helper::add_new_person(),
            helper::Action::PrintAllPerson => {
                let all_person = helper::find_all_person().unwrap();
                println!("{all_person}");
                // helper::export_all_person();
            }
            helper::Action::Quit => {
                println!("Au revoir!");
                break;
            }
            helper::Action::Unknown => println!("Action inconnu!"),
        }
    }
}
