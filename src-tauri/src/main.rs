#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bevy::{app::ScheduleRunnerSettings, prelude::*, utils::Duration};
use crossbeam_channel::{bounded, Sender};
use std::thread;
use tauri::Manager;

struct TauriBridge(Sender<u32>);

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
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            tauri::async_runtime::spawn(async move {
                loop {
                    match rx.try_iter().last() {
                        Some(payload) => {
                            window
                                .emit("send_state", payload)
                                .expect("Event should be sent");
                        }
                        _ => {}
                    }

                    thread::sleep(Duration::from_millis(50));
                }
            });

            Ok(())
        })
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
