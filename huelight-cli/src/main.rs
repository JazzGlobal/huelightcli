use anyhow::Ok;
use clap::{Arg, Parser};
use huelight_core as hue;
use hue::models::LightState;

#[derive(Debug, Clone)]
enum Command {
    CreateUser,
    GetLights,
    SetLightState,
}

struct Args {
    command: String, // TODO: This field is never read from, can we parse an "Args" instance from Clap and match on the command str that way?
    ip_address: String,
    username: Option<String>,
    light_id: Option<u32>,
    light_state: Option<LightState>,
}

async fn run_command(cmd: Command, args: &Args) -> anyhow::Result<()> {
    match cmd {
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
        Command::SetLightState => {
            // Call async function to set light state
            println!("Setting light state...");

            let ip_address = &args.ip_address;
            let username = args
                .username
                .clone()
                .expect("Username is required for setting light state.");

            let client = hue::client::ReqwestHueClient {
                client: reqwest::Client::new(),
            };

            let mut logger = hue::client::Logger { log: Vec::new() };

            hue::client::async_set_light_state(
                ip_address,
                &username,
                args.light_id.expect("Light ID is required for setting light state."),
                &args
                    .light_state
                    .clone()
                    .expect("Light state is required for setting light state."),
                &client,
                &mut logger,
            )
            .await
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // CLI application that will interface with the Philips Hue API to control smart lights with CMD commands.
    let cli = clap::Command::new("huelightcli")
        .version("1.0")
        .author("Christopher J Gambrell")
        .about("Control Philips Hue lights from the command line")
        .arg(
            Arg::new("command")
                .short('c')
                .help("The command to execute: create-user, get-lights, set-light-state")
                .required(true)
        )
        .arg(
            Arg::new("ip_address")
                .short('i')
                .help("The IP address of the Hue Bridge")
                .required(true)
        ).arg(
            Arg::new("username")
                .short('u')
                .help("The username to use in requests for the Hue Bridge API")
                .required(false)
        )
        .arg(
            Arg::new("light_id")
                .short('l')
                .help("The ID of the light to control")
                .required(false)
        )
        .arg(
            Arg::new("light_state")
                .short('s')
                .help("The state to set the light to (e.g., on, off, brightness)")
                .required(false)
        )
        .get_matches();

        let command_str = cli.get_one::<String>("command").unwrap();
        let ip_address = cli.get_one::<String>("ip_address").unwrap();

        match command_str.as_str() {
            "create-user" => {
                let args = Args {
                    command: command_str.clone(),
                    ip_address: ip_address.clone(),
                    username: None,
                    light_id: None,
                    light_state: None,
                };
                run_command(Command::CreateUser, &args).await?;
            }
            "get-lights" => {
                let username = cli.get_one::<String>("username").expect("Username is required for getting lights.").clone();
                let args = Args {
                    command: command_str.clone(),
                    ip_address: ip_address.clone(),
                    username: Some(username),
                    light_id: None,
                    light_state: None,
                };
                run_command(Command::GetLights, &args).await?;
            }
            "set-light-state" => {
                let username = cli.get_one::<String>("username").expect("Username is required for setting light state.").clone();
                let light_id = cli.get_one::<String>("light_id").expect("Light ID is required for setting light state.").parse::<u32>().expect("Light ID must be a valid number.");
                // We're going to make sure light_state can be parsed from JSON.
                let light_state: LightState = serde_json::from_str(cli.get_one::<String>("light_state").expect("Light state is required for setting light state.")).expect("Failed to parse light state JSON");
                let args = Args {
                    command: command_str.clone(),
                    ip_address: ip_address.clone(),
                    username: Some(username),
                    light_id: Some(light_id),
                    light_state: Some(light_state),

                };
                run_command(Command::SetLightState, &args).await?;
            }
            _ => {
                panic!("Unknown command!: {}", command_str);
            }
        }

    Ok(())
}
