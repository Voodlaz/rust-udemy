use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked,
}

pub fn run() {
    println!("\n====3.17 COMBINATION LOCK====");

    let code = String::from("12345");
    // Замок изначально закрыт
    let mut state = State::Locked;
    let mut entry = String::new();
    
    loop {
        println!("Code {} is equal to {} is {}", code, entry, code == entry);
        match state {
            State::Locked => {
                println!("ENTER THE CODE");
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim())
                    }
                    // jump back to loop
                    Err(_) => continue
                }

                if entry == code {
                    println!("entry == code");
                    state = State::Unlocked;
                }
                if code != entry {
                    println!("code != entry");
                    state = State::Failed;
                }

            }
            State::Failed => {
                println!("FAILED!");
                entry.clear();
                state = State::Locked;
            }
            State::Unlocked => {
                println!("UNLOCKED!");
                return;
            }
        }
    }
}