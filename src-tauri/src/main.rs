#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bevy::{app::ScheduleRunnerSettings, prelude::*, utils::Duration};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::thread;

struct TauriBridge(Sender<u32>);

struct BevyBridge(Receiver<u32>);

#[derive(Default)]
struct CounterValue(u32);

fn main() {
    let (tx, rx) = bounded::<u32>(1000);

    thread::spawn(move || {
        App::new()
            .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            )))
            .insert_resource(CounterValue::default())
            .insert_resource(TauriBridge(tx))
            .insert_resource(CounterValue(0))
            .add_plugins(MinimalPlugins)
            .add_system(increment_counter)
            .add_system(send_counter)
            .run()
    });

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(BevyBridge(rx))
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}

fn increment_counter(mut state: ResMut<CounterValue>) {
    state.0 = (state.0 + 1) % 1_000_000u32;
}

fn send_counter(tauri_bridge: ResMut<TauriBridge>, counter: Res<CounterValue>) {
    tauri_bridge
        .0
        .send(counter.0)
        .expect("Failed to send on channel");
}

