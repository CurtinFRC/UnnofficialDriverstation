// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt;

use ds::{DriverStation, Mode};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum AllianceColour {
    Red,
    Blue,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RobotState {
    Disabled,
    Enabled,
    Estopped,
}

#[derive(Debug)]
pub struct RobotMode(Mode);

impl Serialize for RobotMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{:?}", self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RobotMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ModeVisitor;

        impl<'d> Visitor<'d> for ModeVisitor {
            type Value = RobotMode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`secs` or `nanos`")
            }

            fn visit_str<E>(self, value: &str) -> Result<RobotMode, E>
            where
                E: de::Error,
            {
                match value {
                    "Teleop" | "Teleoperated" => Ok(RobotMode(Mode::Teleoperated)),
                    "Auto" | "Autonomous" => Ok(RobotMode(Mode::Autonomous)),
                    "Test" => Ok(RobotMode(Mode::Test)),
                    _ => Err(de::Error::unknown_field(value, MODE_FIELDS)),
                }
            }
        }

        deserializer.deserialize_identifier(ModeVisitor)
    }
}
const MODE_FIELDS: &[&str] = &["Teleop", "Auto", "Test"];

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    colour: AllianceColour,
    position: u8,
    state: RobotState,
    mode: RobotMode,
    team_num: u32,
}

#[tauri::command]
fn send_packet(packet: Packet) {
    assert!(packet.position < 4);
    let mut connection;
    match packet.colour {
        AllianceColour::Red => {
            connection =
                DriverStation::new_team(packet.team_num, ds::Alliance::new_red(packet.position))
        }
        AllianceColour::Blue => {
            connection =
                DriverStation::new_team(packet.team_num, ds::Alliance::new_blue(packet.position))
        }
    }

    match packet.state {
        RobotState::Enabled => connection.enable(),
        RobotState::Disabled => connection.disable(),
        RobotState::Estopped => {
            connection.estop();
            panic!("ESTOPPED HOLY HELL");
        }
    }

    connection.set_mode(packet.mode.0);
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let connection = DriverStation::new_team(9999, ds::Alliance::new_red(0));

    tauri::Builder::default()
        .manage(connection)
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![send_packet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
