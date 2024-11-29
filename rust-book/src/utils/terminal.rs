use rpassword::prompt_password;
use std::io::Write;


pub fn show_menu(title: &str, itens: &[&str], exit_option: bool) -> u32 {
    // clear_screen();

    let full_title = String::from("Rust Book :: ") + title;
    print_header(&full_title);

    show_items(itens);

    println!("{}", if exit_option { "* - Exit" } else { "* - Return" });
    print!("\nEscolha uma opção: ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let chosen_option: Result<u32, _> = line.trim().parse();

    match chosen_option {
        Ok(chosen_option) => chosen_option,
        _ => 0,
    }
}


fn show_items(itens: &[&str]) {
    for (i, item) in itens.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}


pub fn wait_for_enter() {
    prompt_password("Pressione ENTER para continuar...").unwrap();
}


pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}


pub fn print_header(title: &str) {
    let horizontal_line = String::from("=").repeat(title.len());

    println!("\n{horizontal_line}");
    println!("{title}");
    println!("{horizontal_line}\n");
}

pub fn terminate(){
    println!("Bye.");
    std::process::exit(0);
}
