// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fmt, sync::Mutex};

use ds::{DriverStation, Mode};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AllianceColour {
    Red,
    Blue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RobotState {
    Disabled,
    Enabled,
    Estopped,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Packet {
    pub colour: AllianceColour,
    pub position: u8,
    pub state: RobotState,
    pub mode: RobotMode,
    pub team_num: u32,
}

pub struct DriverStationState {
    ds: DriverStation,
    team_num: u32,
    position: u8,
    colour: AllianceColour,
}

#[tauri::command]
fn send_packet(
    last_packet: State<Mutex<Packet>>,
    packet: Packet,
    connection: State<Mutex<DriverStationState>>,
) {
    assert!(packet.position < 4);

    let state = &mut connection.lock().unwrap();
    let change_team_num = state.team_num != packet.team_num;
    let change_position = state.position != packet.position;
    let change_colour = state.colour != packet.colour;
    let ds = &mut state.ds;

    if change_team_num {
        ds.set_team_number(packet.team_num);
    }

    if change_colour || change_position {
        match packet.colour {
            AllianceColour::Red => {
                ds.set_alliance(ds::Alliance::new_red(packet.position));
            }
            AllianceColour::Blue => {
                ds.set_alliance(ds::Alliance::new_blue(packet.position));
            }
        }
    }

    match packet.state {
        RobotState::Enabled => ds.enable(),
        RobotState::Disabled => ds.disable(),
        RobotState::Estopped => {
            ds.estop();
            panic!("ESTOPPED HOLY HELL");
        }
    }

    ds.set_mode(packet.mode.0);

    *last_packet.lock().unwrap() = packet;
}

#[tauri::command]
fn restart_code(state: State<Mutex<DriverStationState>>) {
    state.lock().unwrap().ds.restart_code();
}

#[tauri::command]
fn estop(last_packet: State<Mutex<Packet>>, connection: State<Mutex<DriverStationState>>) {
    let mut packet = last_packet.lock().unwrap().clone();
    packet.state = RobotState::Enabled;
    send_packet(last_packet, packet, connection);
}

#[tauri::command]
fn disable(last_packet: State<Mutex<Packet>>, connection: State<Mutex<DriverStationState>>) {
    let mut packet = last_packet.lock().unwrap().clone();
    packet.state = RobotState::Disabled;
    send_packet(last_packet, packet, connection);
}

#[tauri::command]
fn enable(last_packet: State<Mutex<Packet>>, connection: State<Mutex<DriverStationState>>) {
    let mut packet = last_packet.lock().unwrap().clone();
    packet.state = RobotState::Enabled;
    send_packet(last_packet, packet, connection);
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    #[cfg(target_os = "windows")]
    {
        panic!("Erm what the sigma use the official DS.")
    }

    tauri::Builder::default()
        .manage(Mutex::new(DriverStationState {
            ds: DriverStation::new_team(9999, ds::Alliance::new_red(0)),
            colour: AllianceColour::Red,
            position: 0,
            team_num: 9999,
        }))
        .manage(Mutex::new(Packet {
            colour: AllianceColour::Red,
            mode: RobotMode(Mode::Teleoperated),
            position: 0,
            state: RobotState::Disabled,
            team_num: 9999,
        }))
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![
            send_packet,
            restart_code,
            estop,
            enable,
            disable
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
