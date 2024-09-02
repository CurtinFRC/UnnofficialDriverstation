// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fmt, fs::File, io::Write, thread, time::Duration};

use once_cell::sync::Lazy;
use tokio::sync::{RwLock, RwLockWriteGuard};

use ds::{DriverStation, JoystickValue, Mode};
use gilrs::{Axis, Button, Gilrs};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

use tauri::{api::path::app_config_dir, Manager, State};

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

const MODE_FIELDS: &[&str] = &["Teleop", "Auto", "Test"];

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Packet {
    pub colour: AllianceColour,
    pub position: u8,
    pub state: RobotState,
    pub mode: RobotMode,
    pub team_num: u32,
}

pub struct DriverStationState {
    ds: Option<DriverStation>,
    team_num: u32,
    position: u8,
    colour: AllianceColour,
}

fn send_packet_no_last(
    packet: RwLockWriteGuard<'_, Packet>,
    mut connection: RwLockWriteGuard<'_, DriverStationState>,
) {
    assert!(packet.position < 4);

    let change_team_num = connection.team_num != packet.team_num;
    let change_position = connection.position != packet.position;
    let change_colour = connection.colour != packet.colour;
    let ds = &mut match connection.ds.as_mut() {
        Some(ds) => ds,
        None => return,
    };

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
}

#[tauri::command]
async fn send_packet(
    last_packet: State<'_, Lazy<RwLock<Packet>>>,
    packet: Packet,
) -> Result<(), ()> {
    *last_packet.write().await = packet;

    Ok(())
}

#[tauri::command]
async fn restart_code(state: State<'_, Lazy<RwLock<DriverStationState>>>) -> Result<(), ()> {
    match state.write().await.ds.as_mut() {
        Some(ds) => ds.restart_code(),
        None => {
            println!("Can't restart code, DriverStationState doesn't exist yet.");
            return Ok(());
        }
    }

    Ok(())
}

#[tauri::command]
async fn estop(last_packet: State<'_, Lazy<RwLock<Packet>>>) -> Result<(), ()> {
    let mut packet = last_packet.write().await.clone();
    packet.state = RobotState::Enabled;
    send_packet(last_packet, packet).await.unwrap();

    Ok(())
}

#[tauri::command]
async fn disable(last_packet: State<'_, Lazy<RwLock<Packet>>>) -> Result<(), ()> {
    let mut packet = last_packet.write().await.clone();
    packet.state = RobotState::Disabled;
    send_packet(last_packet, packet).await.unwrap();

    Ok(())
}

#[tauri::command]
async fn enable(last_packet: State<'_, Lazy<RwLock<Packet>>>) -> Result<(), ()> {
    let mut packet = last_packet.write().await.clone();
    packet.state = RobotState::Enabled;
    send_packet(last_packet, packet).await.unwrap();

    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

static LAST_PACKET: Lazy<RwLock<Packet>> = Lazy::new(|| {
    RwLock::new(Packet {
        colour: AllianceColour::Red,
        mode: RobotMode(Mode::Teleoperated),
        position: 1,
        state: RobotState::Disabled,
        team_num: 4788,
    })
});

static DRIVERSTATION_STATE: Lazy<RwLock<DriverStationState>> = Lazy::new(|| {
    RwLock::new(DriverStationState {
        ds: None,
        colour: AllianceColour::Red,
        position: 1,
        team_num: 4788,
    })
});

const AXIS: [Axis; 4] = [
    Axis::RightStickX,
    Axis::RightStickY,
    Axis::LeftStickX,
    Axis::LeftStickY,
];

const BUTTONS: [Button; 8] = [
    Button::South,
    Button::East,
    Button::North,
    Button::West,
    Button::LeftTrigger,
    Button::RightTrigger,
    Button::LeftThumb,
    Button::RightThumb,
];

// TODO: Send these
const _POVS: [Button; 4] = [
    Button::DPadUp,
    Button::DPadDown,
    Button::DPadRight,
    Button::DPadLeft,
];

fn main() {
    #[cfg(target_os = "windows")]
    {
        panic!("Erm what the sigma use the official DS.")
    }

    tauri::Builder::default()
        .setup(|app| {
            let config_dir =
                app_config_dir(&app.config()).expect("Should be able to get config dir.");
            let _ = std::fs::create_dir_all(&config_dir);
            let location = config_dir
                .to_str()
                .expect("Config directory should be valid string.")
                .to_string()
                + "/team_num";
            let team_num_string = std::fs::read_to_string(&location);
            let team_num: u32 = match team_num_string {
                Ok(string) => string
                    .parse()
                    .expect("Saved team number should be a number."),
                Err(_) => {
                    File::create_new(&location)
                        .expect("Team number file doesn't exist, so should be creatable.")
                        .write_all(b"4788")
                        .expect("Should be able to write to team number file.");
                    4788
                }
            };

            tauri::async_runtime::spawn(async move {
                LAST_PACKET.write().await.team_num = team_num;
                let mut ds = DRIVERSTATION_STATE.write().await;
                ds.ds = Some(DriverStation::new_team(team_num, ds::Alliance::new_red(1)));
                ds.team_num = team_num;
                let driverstation = ds.ds.as_mut().expect("Driverstation should exist.");
                driverstation.set_use_usb(false);
                driverstation.set_joystick_supplier(|| {
                    let gilrs = Gilrs::new().expect("Should be able to load Gilrs.");
                    let mut out: Vec<Vec<JoystickValue>> = vec![];
                    for (_id, gamepad) in gilrs.gamepads() {
                        let mut values: Vec<JoystickValue> = vec![];
                        for i in BUTTONS {
                            values.push(JoystickValue::Button {
                                id: i as u8,
                                pressed: gamepad.is_pressed(i),
                            })
                        }

                        for i in AXIS {
                            values.push(JoystickValue::Axis {
                                id: i as u8,
                                value: gamepad.value(i),
                            })
                        }

                        out.push(values);
                    }
                    out
                });
            });

            app.manage(&DRIVERSTATION_STATE);
            app.manage(&LAST_PACKET);

            tauri::async_runtime::spawn(async move {
                loop {
                    send_packet_no_last(
                        LAST_PACKET.write().await,
                        DRIVERSTATION_STATE.write().await,
                    );
                    thread::sleep(Duration::from_millis(15));
                }
            });

            Ok(())
        })
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
