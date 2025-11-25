use anyhow::Result;
use std::io::{self, Write};

pub fn repl() -> Result<()> {
    println!("Rustainer REPL v0.1.0");
    println!("Type 'help' for commands, 'exit' to quit\n");

    loop {
        print!("rustainer> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];

        match cmd {
            "help" | "?" => print_help(),
            "exit" | "quit" | "q" => {
                println!("Goodbye!");
                break;
            }
            "version" | "v" => println!("rustainer v0.1.0"),
            "list" | "ls" => list_containers(),
            "images" => list_images(),
            "ps" => list_running(),
            "info" => print_info(),
            "clear" => clear_screen(),
            _ => {
                println!("Unknown command: '{}'. Type 'help' for available commands.", cmd);
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!(
        r#"
Available commands:
  help, ?       Show this help message
  exit, quit, q Exit the REPL
  version, v    Show version information
  list, ls      List all containers
  images        List downloaded images
  ps            List running containers
  info          Show system information
  clear         Clear the screen
"#
    );
}

fn list_containers() {
    println!("CONTAINER ID    IMAGE       STATUS      NAME");
    println!("-------------------------------------------");
    println!("(no containers)");
}

fn list_images() {
    println!("REPOSITORY      TAG         IMAGE ID        SIZE");
    println!("------------------------------------------------");
    println!("(no images)");
}

fn list_running() {
    println!("CONTAINER ID    IMAGE       STATUS      COMMAND");
    println!("-----------------------------------------------");
    println!("(no running containers)");
}

fn print_info() {
    println!("Rustainer Info:");
    println!("  Version:    0.1.0");
    println!("  OS:         {}", std::env::consts::OS);
    println!("  Arch:       {}", std::env::consts::ARCH);
    
    #[cfg(target_os = "linux")]
    println!("  Features:   namespaces, cgroups, chroot");
    
    #[cfg(all(unix, not(target_os = "linux")))]
    println!("  Features:   chroot (limited mode)");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

