use anyhow::{Context, Ok};
use clap::Arg;
use hue::logger::{ILogger, Logger};
use hue::models::LightState;
use huelight_core::{self as hue};
#[derive(Debug, Clone)]
enum Command {
    CreateUser,
    GetLights,
    SetLightState,
}

struct Args {
    // command: String, // TODO: This field is never read from, can we parse an "Args" instance from Clap and match on the command str that way?
    ip_address: String,
    username: Option<String>,
    light_id: Option<u32>,
    light_state: Option<LightState>,
}

async fn run_command(cmd: Command, args: &Args) -> anyhow::Result<()> {
    let mut logger = Logger::default();

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

            hue::hue_api::async_create_user(ip_address, username, &client, &mut logger).await
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

            hue::hue_api::async_get_all_lights(ip_address, &username, &client, &mut logger).await
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

            hue::hue_api::async_set_light_state(
                ip_address,
                &username,
                args.light_id
                    .expect("Light ID is required for setting light state."),
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
                .required(true),
        )
        .arg(
            Arg::new("ip_address")
                .short('i')
                .help("The IP address of the Hue Bridge")
                .required(false),
        )
        .arg(
            Arg::new("username")
                .short('u')
                .help("The username to use in requests for the Hue Bridge API")
                .required(false),
        )
        .arg(
            Arg::new("light_id")
                .short('l')
                .help("The ID of the light to control")
                .required(false),
        )
        .arg(
            Arg::new("light_state")
                .short('s')
                .help("The state to set the light to (e.g., on, off, brightness)")
                .required(false),
        )
        .arg(
            Arg::new("use_config")
                .short('f')
                .help("Use saved configuration from config file")
                .required(false),
        )
        .get_matches();

    let command_str = cli.get_one::<String>("command").unwrap();
    let mut ip_address: String = cli
        .get_one::<String>("ip_address")
        .unwrap_or(&"".to_string().clone())
        .clone();
    let mut username: String = cli
        .get_one::<String>("username")
        .unwrap_or(&"".to_string().clone())
        .clone();

    // ip and user will either have a value, or be empty strings at this point.

    // If use config is set, load config from file.
    let config = hue::config::Config::load().await;
    if config.is_err() {
        println!("No saved configuration found. Proceeding without config file.");
    }

    let use_config: String = cli
        .get_one::<String>("use_config")
        .unwrap_or(&"false".to_string().clone())
        .clone();
    if use_config == "true" && config.is_ok() {
        println!("Using saved configuration from config file...");

        // Override ip and user with the loaded config values
        let c = config.unwrap();
        ip_address = c.bridge_ip.clone();
        username = c.username.clone();
    }

    let mut logger = Logger::default();

    // A username and password are required for all commands, but we want special error messages for setup because they are required to be provided by the user via command line for that command.
    if command_str != "setup" {
        if ip_address.is_empty() {
            anyhow::bail!(
                "IP address is required. Please provide it via command line or config file."
            );
        }
        if username.is_empty() {
            anyhow::bail!(
                "Username is required for all commands besides 'setup'. Please provide it via command line or config file."
            );
        }
    }

    match command_str.as_str() {
        "setup" => {
            logger.log("Setting up configuration...");
            let ip_address = cli.get_one::<String>("ip_address").context("ip address is required and must be supplied via command line for the setup command.")?;
            let username = cli.get_one::<String>("username").context(
                "username is required and must be supplied via command line for the setup command.",
            )?;
            let config = hue::config::Config::new(ip_address.clone(), username.clone());
            config.save(&mut logger).await?;
            println!("Config saved: {:?}", config);
        }
        "create-user" => {
            let username = cli.get_one::<String>("username").context("username is required and must be supplied via command line for the create-user command.")?;
            let args = Args {
                ip_address,
                username: Some(username.clone()),
                light_id: None,
                light_state: None,
            };
            run_command(Command::CreateUser, &args).await?;
        }
        "get-lights" => {
            let args = Args {
                ip_address,
                username: Some(username.clone()),
                light_id: None,
                light_state: None,
            };

            run_command(Command::GetLights, &args).await?;
        }
        "set-light-state" => {
            let light_id = cli
                .get_one::<String>("light_id")
                .expect("Light ID is required for setting light state.")
                .parse::<u32>()
                .expect("Light ID must be a valid number.");
            // We're going to make sure light_state can be parsed from JSON.
            let light_state: LightState = serde_json::from_str(
                cli.get_one::<String>("light_state")
                    .expect("Light state is required for setting light state."),
            )
            .expect("Failed to parse light state JSON");
            let args = Args {
                ip_address,
                username: Some(username),
                light_id: Some(light_id),
                light_state: Some(light_state),
            };
            run_command(Command::SetLightState, &args).await?;
        }
        _ => {
            anyhow::bail!("Unknown command!: {}", command_str);
        }
    }

    Ok(())
}
