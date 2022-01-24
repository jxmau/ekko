fn main() {

    // Parse the args at main
    let args : Vec<String> = std::env::args().collect();

    // TODO: Perhaps change the expect.
    let cmd = args.get(1).expect("A command is required - Type help to show the list of commands.");

    match cmd as &str {
        
        "debug" => println!("Debugging"),
        "help" | _  => print!("This is help."),
    }

}




// Fetch The file
