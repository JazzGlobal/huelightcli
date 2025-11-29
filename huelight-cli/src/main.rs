use hue::logger::{ILogger, Logger};
use hue::models::light::LightState;
use huelight_core::client::ReqwestHueClient;
use huelight_core::error::{CoreError, HueBridgeError};
use huelight_core::models::hueerror::HueResponseEntry;
use huelight_core::{self as hue, hue_api};

pub mod error;
use error::CLIError;

#[tokio::main]
async fn main() -> Result<(), CLIError> {
    // CLI application that will interface with the Philips Hue API to control smart lights with CMD commands.
    let cli = clap::Command::new("huelightcli")
        .version("1.0")
        .author("Christopher J Gambrell")
        .about("Control Philips Hue lights from the command line")
        .subcommand(
            clap::Command::new("setup")
                .about("Provides commands necessary for configuring the Hue Bridge for light control.")
                .subcommand(clap::Command::new("config")
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
                .subcommand(
                    clap::Command::new("hue")
                    .about("Sets the hue for a light")
                    .arg(
                        clap::Arg::new("light_id")
                            .required(true)
                            .help("ID of the light to set brightness")
                    )
                    .arg(
                        clap::Arg::new("hue")
                        .required(true)
                        .help("Value between 0-65535 to set the light hue to. This is a wrapping value. Both 0 and 65535 are red. 25500 is green and 46920 is blue.")
                    )
                )
                .subcommand(
                    clap::Command::new("saturation")
                    .about("Sets the saturation for a light")
                    .arg(
                        clap::Arg::new("light_id")
                            .required(true)
                            .help("ID of the light to set brightness")
                    )
                    .arg(
                        clap::Arg::new("saturation")
                        .required(true)
                        .help("Value between 0-255 to set the light saturation to. 254 is the most saturated (colored) and 0 is the least saturated (white).")
                    )
                )
        )
        .get_matches();

    let mut logger = Logger::default();

    let r_client = reqwest::Client::new();
    let client = ReqwestHueClient::new(r_client);

    let config: Result<hue::config::Config, CLIError> = match cli.subcommand_name() {
        Some(name) if name != "setup" => {
            Ok(hue::config::Config::load(&hue::config::TokioFileHandler).await?)
        }
        _ => Err(CLIError::ConfigNotLoaded),
    };

    if config
        .as_ref()
        .map_or(true, |c| c.username.is_empty() || c.bridge_ip.is_empty())
        && cli.subcommand_name() != Some("setup")
    {
        return Err(CLIError::ConfigNotLoaded);
    }

    // if we get here, we have a valid config or are running setup
    let c = config.unwrap_or(hue::config::Config {
        bridge_ip: String::new(),
        username: String::new(),
    });

    return match cli.subcommand() {
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
                    .await
                    .map_err(CLIError::HueLightCoreError)?;

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

                    Ok(())
                }
                Some(("on", light_cmd)) => {
                    let light_id = light_cmd
                        .get_one::<String>("light_id")
                        .expect("Light ID is required")
                        .parse::<u32>()
                        .expect("Light ID must be a number");
                    println!("Turning light {} on...", light_id);
                    let light_state = LightState::default().with_on(true);
                    hue_api::async_set_light_state(
                        &c.bridge_ip,
                        &c.username,
                        light_id,
                        &light_state,
                        &client,
                    )
                    .await
                    .map_err(CLIError::HueLightCoreError)?;
                    Ok(())
                }
                Some(("off", light_cmd)) => {
                    let light_id = light_cmd
                        .get_one::<String>("light_id")
                        .expect("Light ID is required")
                        .parse::<u32>()
                        .expect("Light ID must be a number");
                    println!("Turning light {} off...", light_id);
                    let light_state = LightState::default().with_on(false);
                    hue_api::async_set_light_state(
                        &c.bridge_ip,
                        &c.username,
                        light_id,
                        &light_state,
                        &client,
                    )
                    .await
                    .map_err(CLIError::HueLightCoreError)?;
                    Ok(())
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
                    .await
                    .map_err(CLIError::HueLightCoreError)?;

                    if let Some(light) = lights.0.get(&light_id) {
                        let new_state = !light.state.on.unwrap_or(false);
                        let light_state = LightState::default().with_on(new_state);
                        let response = hue_api::async_set_light_state(
                            &c.bridge_ip,
                            &c.username,
                            light_id,
                            &light_state,
                            &client,
                        )
                        .await
                        .map_err(CLIError::HueLightCoreError)?;

                        let success_str = format!("/lights/{}/state/on", light_id);
                        let result_of_toggle = response.iter().find_map(|entry| match entry {
                            HueResponseEntry::Success { success }
                                if success.contains_key(&success_str) =>
                            {
                                Some(success)
                            }
                            _ => None,
                        });

                        let message = if result_of_toggle.is_none() {
                            format!("Failed to toggle light {}!", light_id)
                        } else {
                            format!("Successfully toggled the light {}!", light_id)
                        };
                        logger.log(&message);
                    } else {
                        return Err(CLIError::HueLightCoreError(CoreError::Bridge(
                            HueBridgeError::LightNotFound,
                        )));
                    }

                    Ok(())
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

                    let l_state = LightState::default().with_brightness(brightness);

                    hue_api::async_set_light_state(
                        &c.bridge_ip,
                        &c.username,
                        light_id,
                        &l_state,
                        &client,
                    )
                    .await
                    .map_err(CLIError::HueLightCoreError)?;

                    Ok(())
                }
                Some(("hue", light_cmd)) => {
                    let light_id = light_cmd
                        .get_one::<String>("light_id")
                        .expect("Light ID is required")
                        .parse::<u32>()
                        .expect("Light ID must be a number");
                    let hue = light_cmd
                        .get_one::<String>("hue")
                        .expect("Hue is required")
                        .parse::<u16>()
                        .expect("Hue must be a number");
                    
                    println!("Changing light saturation {}...", light_id);
                    let l_state = LightState::default().with_hue(hue);

                    hue_api::async_set_light_state(
                        &c.bridge_ip,
                        &c.username,
                        light_id,
                        &l_state,
                        &client,
                    )
                    .await
                    .map_err(CLIError::HueLightCoreError)?;

                    Ok(())
                }
                Some(("saturation", light_cmd)) => {
                    let light_id = light_cmd
                        .get_one::<String>("light_id")
                        .expect("Light ID is required")
                        .parse::<u32>()
                        .expect("Light ID must be a number");
                    let saturation = light_cmd
                        .get_one::<String>("saturation")
                        .expect("Saturation is required")
                        .parse::<u8>()
                        .expect("Saturation must be a number");
                    
                    println!("Changing light saturation {}...", light_id);
                    let l_state = LightState::default().with_saturation(saturation);

                    hue_api::async_set_light_state(
                        &c.bridge_ip,
                        &c.username,
                        light_id,
                        &l_state,
                        &client,
                    )
                    .await
                    .map_err(CLIError::HueLightCoreError)?;

                    Ok(())
                }
                _ => Err(CLIError::InvalidCommandError),
            }
        }
        Some(("setup", sub_setup_cmd)) => {
            match sub_setup_cmd.subcommand() {
                Some(("config", setup_config_cmd)) => {
                    // Setup config command
                    let ip_address = setup_config_cmd
                        .get_one::<String>("ip_address")
                        .expect("IP address is required")
                        .to_string();
                    let username = setup_config_cmd
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
                        .await
                        .map_err(CLIError::HueLightCoreError)?;
                    Ok(())
                }
                _ => Err(CLIError::InvalidCommandError),
            }
        }
        _ => Err(CLIError::InvalidCommandError),
    };
}
