use anyhow::Ok;
use hue::logger::{ILogger, Logger};
use hue::models::LightState;
use huelight_core::{self as hue, client, hue_api};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // CLI application that will interface with the Philips Hue API to control smart lights with CMD commands.
    let cli = clap::Command::new("huelightcli")
        .version("1.0")
        .author("Christopher J Gambrell")
        .about("Control Philips Hue lights from the command line")
        .subcommand(
            clap::Command::new("setup")
                .about("Configures the IP address and username for the Hue Bridge, saving them to a config file.")
                .arg(
                    clap::Arg::new("ip_address")
                        .required(true)
                        .short('i')
                        .help("IP address of the Hue Bridge")
                )
                .arg(
                    clap::Arg::new("username")
                        .required(true)
                        .short('u')
                        .help("Username for the Hue Bridge")
                )
        )
        .subcommand(
        clap::Command::new("light")
                .about("Commands to control lights")
                .subcommand(
                    clap::Command::new("list")
                        .about("Get the list of lights connected to the Hue Bridge"),
                )
                .subcommand(
                    clap::Command::new("on")
                        .about("Turn a light on")
                        .arg(
                            clap::Arg::new("light_id")
                                .required(true)
                                .help("ID of light to turn on")
                        ),
                )
                .subcommand(
                    clap::Command::new("off")
                        .about("Turn a light off")
                        .arg(
                            clap::Arg::new("light_id")
                                .required(true)
                                .help("ID of light to turn off")
                        ),
                )
                .subcommand(
                    clap::Command::new("toggle")
                        .about("Toggle a light on or off")
                        .arg(
                            clap::Arg::new("light_id")
                                .required(true)
                                .help("ID of light to toggle")
                        ),
                )
                .subcommand(
                    clap::Command::new("brightness")
                    .about("Sets the brightness for a light")
                    .arg(
                        clap::Arg::new("light_id")
                            .required(true)
                            .help("ID of the light to set brightness")
                    )
                    .arg(
                        clap::Arg::new("brightness")
                        .required(true)
                        .short('b')
                        .help("Value between 0-255 to set light brightness to")
                    )
                )
        )
        .get_matches();

    let mut logger = Logger::default();
    let client = client::ReqwestHueClient {
        client: reqwest::Client::new(),
    };

    let config: anyhow::Result<hue::config::Config> =
        if cli.subcommand_name() != Some("setup") && cli.subcommand_name().is_some() {
            hue::config::Config::load(&hue::config::TokioFileHandler).await
        } else {
            Err(anyhow::anyhow!("No config loaded"))
        };

    if (config.is_err()
        || config.as_ref().unwrap().username.is_empty()
        || config.as_ref().unwrap().bridge_ip.is_empty())
        && cli.subcommand_name() != Some("setup")
    {
        anyhow::bail!("No configuration found! Please run the 'setup' command first.");
    }

    // if we get here, we have a valid config or are running setup
    let c = config.unwrap_or(hue::config::Config {
        bridge_ip: String::new(),
        username: String::new(),
    });

    match cli.subcommand() {
        Some(("light", sub_light_cmd)) => {
            match sub_light_cmd.subcommand() {
                Some(("list", _)) => {
                    // Get the list of lights
                    println!("Getting list of lights...");
                    let lights = hue_api::async_get_all_lights(
                        &c.bridge_ip,
                        &c.username,
                        &client,
                        &mut logger,
                    )
                    .await?;
                    for (id, light) in lights.0 {
                        logger.log(&format!(
                        "Light ID: {}, On: {}, Name: {}, Type: {}, Brightness: {}, Hue: {}, Saturation: {}",
                        id,
                        light.state.on.unwrap_or(false),
                        light.name,
                        light._type,
                        light.state.brightness.unwrap_or(0),
                        light.state.hue.unwrap_or(0),
                        light.state.saturation.unwrap_or(0)
                    ));
                    }
                }
                Some(("on", light_cmd)) => {
                    let light_id = light_cmd
                        .get_one::<String>("light_id")
                        .expect("Light ID is required")
                        .parse::<u32>()
                        .expect("Light ID must be a number");
                    println!("Turning light {} on...", light_id);
                    let light_state = LightState {
                        on: Some(true),
                        ..Default::default()
                    };
                    hue_api::async_set_light_state(
                        &c.bridge_ip,
                        &c.username,
                        light_id,
                        &light_state,
                        &client,
                        &mut logger,
                    )
                    .await?;
                }
                Some(("off", light_cmd)) => {
                    let light_id = light_cmd
                        .get_one::<String>("light_id")
                        .expect("Light ID is required")
                        .parse::<u32>()
                        .expect("Light ID must be a number");
                    println!("Turning light {} off...", light_id);
                    let light_state = LightState {
                        on: Some(false),
                        ..Default::default()
                    };
                    hue_api::async_set_light_state(
                        &c.bridge_ip,
                        &c.username,
                        light_id,
                        &light_state,
                        &client,
                        &mut logger,
                    )
                    .await?;
                }
                Some(("toggle", light_cmd)) => {
                    let light_id = light_cmd
                        .get_one::<String>("light_id")
                        .expect("Light ID is required")
                        .parse::<u32>()
                        .expect("Light ID must be a number");
                    println!("Toggling light {}...", light_id);
                    // Implement logic to toggle light here
                    let lights = hue_api::async_get_all_lights(
                        &c.bridge_ip,
                        &c.username,
                        &client,
                        &mut logger,
                    )
                    .await?;

                    if let Some(light) = lights.0.get(&light_id) {
                        let new_state = !light.state.on.unwrap_or(false);
                        let light_state = LightState {
                            on: Some(new_state),
                            ..Default::default()
                        };
                        hue_api::async_set_light_state(
                            &c.bridge_ip,
                            &c.username,
                            light_id,
                            &light_state,
                            &client,
                            &mut logger,
                        )
                        .await?;
                    } else {
                        anyhow::bail!("Light ID {} not found!", light_id);
                    }
                }
                Some(("brightness", light_cmd)) => {
                    let light_id = light_cmd
                        .get_one::<String>("light_id")
                        .expect("Light ID is required")
                        .parse::<u32>()
                        .expect("Light ID must be a number");
                    println!("Changing light brightness {}...", light_id);
                    let brightness = light_cmd
                        .get_one::<String>("brightness")
                        .expect("Brightness is required")
                        .parse::<u16>()
                        .expect("Brightness must be a number");

                    let l_state = LightState {
                        brightness: Some(brightness),
                        ..Default::default()
                    };

                    hue_api::async_set_light_state(
                        &c.bridge_ip,
                        &c.username,
                        light_id,
                        &l_state,
                        &client,
                        &mut logger,
                    )
                    .await?
                }
                _ => anyhow::bail!("Not a valid light subcommand!"),
            }
        }
        Some(("setup", setup_cmd)) => {
            // Setup command
            let ip_address = setup_cmd
                .get_one::<String>("ip_address")
                .expect("IP address is required")
                .to_string();
            let username = setup_cmd
                .get_one::<String>("username")
                .expect("Username is required")
                .to_string();
            logger.log("Setting up configuration...");
            logger.log(&format!(
                "IP Address: {}, Username: {}",
                ip_address, username
            ));

            huelight_core::config::Config::new(ip_address, username)
                .save(&mut logger, &hue::config::TokioFileHandler)
                .await?;
        }
        Some((_, _)) | None => anyhow::bail!("Not a valid command!"),
    }

    Ok(())
}
