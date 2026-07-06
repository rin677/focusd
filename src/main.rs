use std::io;

fn get_input(v: &mut String) {
  io::stdin().read_line(v).expect("Failed to read input");
  *v = v.trim().to_string()
}

fn main() {
  loop {
    println!("What do you want to do?");
    println!("7. quit app");
    println!();

    let mut selection: String = String::new();
    get_input(&mut selection);
    match selection.to_lowercase().as_str() {
      "q" | "7" | "quit" | "exit" => break,
      _ => println!("not valid command"),
    }
  }
}
