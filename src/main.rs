use clap::Parser;

mod hue;

#[derive(Debug, Clone)]
enum Command {
    CreateUser,
}

#[derive(Parser, Debug)]
struct Args {
    command: String,
}

fn match_command(cmd: &str) -> Option<Command> {
    match cmd {
        "create-user" => Some(Command::CreateUser),
        _ => panic!("Unknown command!: {}", cmd),
    }
}

async fn run_command(cmd: Command) -> anyhow::Result<()> {
    return match cmd {
        Command::CreateUser => {
            // Call async function to create user
            println!("Creating a new user...");

            let ip_address = "<IP_ADDRESS>"; // Pull from arguments
            let device_name = "huelightcli#cli"; // Pull from arguments 

            let client = hue::client::ReqwestHueClient {
                client: reqwest::Client::new(),
            };

            let mut logger = hue::client::Logger { log: Vec::new() };

            hue::client::async_create_user(ip_address, device_name, &client, &mut logger).await
        }
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // CLI application that will interface with the Philips Hue API to control smart lights with CMD commands.
    println!("Welcome to the Philips Hue CLI Controller!");

    let args = Args::parse();
    run_command(match_command(&args.command).unwrap()).await
}
