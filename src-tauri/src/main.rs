// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ds::{DriverStation, Mode};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub enum RobotMode {
    Teleop,
    Auto,
    Test,
}

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

    match packet.mode {
        RobotMode::Auto => connection.set_mode(Mode::Autonomous),
        RobotMode::Teleop => connection.set_mode(Mode::Teleoperated),
        RobotMode::Test => connection.set_mode(Mode::Test),
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![send_packet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
