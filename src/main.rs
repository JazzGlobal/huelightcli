use clap::Parser;

mod hue;

#[derive(Debug, Clone)]
enum Command {
    CreateUser,
    GetLights,
}

#[derive(Parser, Debug)]
struct Args {
    command: String,
    ip_address: String,
    username: Option<String>,
}

fn match_command(cmd: &str) -> Option<Command> {
    match cmd {
        "create-user" => Some(Command::CreateUser),
        "get-lights" => Some(Command::GetLights),
        _ => panic!("Unknown command!: {}", cmd),
    }
}

async fn run_command(cmd: Command, args: &Args) -> anyhow::Result<()> {
    return match cmd {
        Command::CreateUser => {
            // Call async function to create user
            println!("Creating a new user...");

            let ip_address = &args.ip_address;
            let username = &args
                .username
                .clone()
                .unwrap_or_else(|| "defaultuser".to_string());

            let client = hue::client::ReqwestHueClient {
                client: reqwest::Client::new(),
            };

            let mut logger = hue::client::Logger { log: Vec::new() };

            hue::client::async_create_user(ip_address, username, &client, &mut logger).await
        }
        Command::GetLights => {
            // Call async function to get lights
            println!("Getting lights...");

            let ip_address = &args.ip_address;
            let username = args
                .username
                .clone()
                .expect("Username is required for getting lights.");

            let client = hue::client::ReqwestHueClient {
                client: reqwest::Client::new(),
            };

            let mut logger = hue::client::Logger { log: Vec::new() };

            hue::client::async_get_all_lights(ip_address, &username, &client, &mut logger).await
        }
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // CLI application that will interface with the Philips Hue API to control smart lights with CMD commands.
    println!("Welcome to the Philips Hue CLI Controller!");

    let args = Args::parse();
    print!("Running command: {}\n", args.command);
    run_command(match_command(&args.command).unwrap(), &args).await
}
