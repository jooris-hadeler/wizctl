mod types;

use std::{
    net::{IpAddr, SocketAddr, UdpSocket},
    time::Duration,
};

use clap::Parser;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use types::{
    ColorParams, CommandResult, Method, Request, Response, Scene, SceneParams, StateParams,
};

#[derive(Debug, clap::Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    /// The command we want to issue to the light strip(s).
    #[clap(subcommand)]
    command: Command,

    /// The IP Addresses of the light strip(s).
    #[clap(short = 'i', long = "ip", value_names=["IP"])]
    ips: Vec<IpAddr>,
}

#[derive(Debug, clap::Subcommand, Clone)]
pub enum Command {
    /// Sets the dynamic scene the light strip(s) use(s).
    Dynamic {
        /// The scene we want to use.
        #[clap(value_enum)]
        scene: Scene,

        /// The speed at wich we want the scene to play.
        #[clap(default_value_t = 20)]
        speed: u8,

        /// The brightness we want the scene to be played at.
        #[clap(default_value_t = 75)]
        brightness: u8,
    },

    /// Sets the static color the light strip(s) use(s).
    Static {
        /// The red component.
        red: u8,

        /// The green component.
        green: u8,

        /// The blue component.
        blue: u8,

        /// The brightness we want the color to be displayed at.
        #[clap(default_value_t = 75)]
        brightness: u8,
    },

    /// Turns on the light strip(s).
    On,

    /// Turns off the light strip(s).
    Off,
}

#[derive(Debug, Error)]
pub enum WizError {
    #[error("{0}")]
    Custom(&'static str),

    #[error("failed to serialize/deserialize JSON data")]
    Serde(#[from] serde_json::Error),

    #[error("failed to send/receive data")]
    IO(#[from] std::io::Error),

    #[error("failed to convert from/to UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub struct WizLightStrip {
    ip: IpAddr,
}

impl WizLightStrip {
    pub fn new(ip: impl Into<IpAddr>) -> Self {
        Self { ip: ip.into() }
    }

    fn send_request<R>(&self, command: impl Serialize) -> Result<R, WizError>
    where
        R: DeserializeOwned,
    {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        socket.connect(SocketAddr::new(self.ip, 38899))?;

        socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        let command = serde_json::to_string(&command)?;

        socket.send(command.as_bytes())?;

        let mut buffer = [0; 2048];
        let size = socket.recv(&mut buffer)?;

        let json = String::from_utf8(buffer[..size].to_vec())?;

        Ok(serde_json::from_str(&json)?)
    }

    pub fn turn_on(&self) -> Result<bool, WizError> {
        Ok(self
            .send_request::<Response<CommandResult>>(Request::new_without_id(
                Method::SetState,
                StateParams { state: true },
            ))?
            .result
            .success)
    }

    pub fn turn_off(&self) -> Result<bool, WizError> {
        Ok(self
            .send_request::<Response<CommandResult>>(Request::new_without_id(
                Method::SetState,
                StateParams { state: false },
            ))?
            .result
            .success)
    }

    pub fn set_color(&self, red: u8, green: u8, blue: u8, dimming: u8) -> Result<bool, WizError> {
        if !matches!(dimming, 10..=100) {
            return Err(WizError::Custom("dimming value must be between 10 and 100"));
        }

        Ok(self
            .send_request::<Response<CommandResult>>(Request::new(
                1,
                Method::SetPilot,
                ColorParams {
                    red,
                    green,
                    blue,
                    dimming,
                },
            ))?
            .result
            .success)
    }

    pub fn set_scene(&self, scene: Scene, speed: u8, dimming: u8) -> Result<bool, WizError> {
        if !matches!(dimming, 10..=100) {
            return Err(WizError::Custom("dimming value must be between 10 and 100"));
        }

        if !matches!(speed, 10..=200) {
            return Err(WizError::Custom("speed value must be between 10 and 200"));
        }

        Ok(self
            .send_request::<Response<CommandResult>>(Request::new(
                1,
                Method::SetPilot,
                SceneParams {
                    scene,
                    speed,
                    dimming,
                },
            ))?
            .result
            .success)
    }
}

fn main() -> Result<(), WizError> {
    let Options { ips: ip, command } = Options::parse();

    let lights: Vec<WizLightStrip> = ip.into_iter().map(WizLightStrip::new).collect();

    match command {
        Command::Dynamic {
            scene,
            speed,
            brightness,
        } => {
            for light in lights.iter() {
                light.set_scene(scene, speed, brightness)?;
            }
        }
        Command::Static {
            red,
            green,
            blue,
            brightness,
        } => {
            for light in lights.iter() {
                light.set_color(red, green, blue, brightness)?;
            }
        }
        Command::On => {
            for light in lights.iter() {
                light.turn_on()?;
            }
        }
        Command::Off => {
            for light in lights.iter() {
                light.turn_off()?;
            }
        }
    }

    Ok(())
}
