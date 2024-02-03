use std::{process, sync::Arc, thread, time::Duration};

use scripts::Script;
use tokio::select;
use tracing::info;

use crate::controllers::{launchpad_mini_mk3::LaunchpadMiniMk3, Alles, Controller};

mod controllers;
mod scripts;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Initializing Rustversi on Launchpad Mini Mk3...");
    info!("Rules: https://en.wikipedia.org/wiki/Reversi");
    info!("Press User to pass a turn");
    info!("Press Session to count points");

    let mut controllers: Vec<Arc<Box<dyn Alles>>> = Vec::new();

    let controller: Arc<Box<dyn Alles>> =
        Arc::new(LaunchpadMiniMk3::guess().unwrap());
    controllers.push(controller.clone());
    controller.initialize().unwrap();

    // info!("Successfully started controller: {}", controller.name());

    // state.controllers.push(controller);

    let mut script2 = scripts::othello::PingScript::new();

    let controller21 = controller.clone();
    tokio::spawn(async move { controller21.run(&mut script2).unwrap() });

    select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received SIGINT, shutting down");
        },
    }
    controller.clear().unwrap();

    thread::sleep(Duration::from_millis(100));

    process::exit(0);
}
