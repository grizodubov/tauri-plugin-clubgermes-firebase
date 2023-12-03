use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::ClubgermesFirebase;
#[cfg(mobile)]
use mobile::ClubgermesFirebase;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the clubgermes-firebase APIs.
pub trait ClubgermesFirebaseExt<R: Runtime> {
  fn clubgermes_firebase(&self) -> &ClubgermesFirebase<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ClubgermesFirebaseExt<R> for T {
  fn clubgermes_firebase(&self) -> &ClubgermesFirebase<R> {
    self.state::<ClubgermesFirebase<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("clubgermes-firebase")
    .invoke_handler(tauri::generate_handler![commands::execute])
    .setup(|app, api| {
      #[cfg(mobile)]
      let clubgermes_firebase = mobile::init(app, api)?;
      #[cfg(desktop)]
      let clubgermes_firebase = desktop::init(app, api)?;
      app.manage(clubgermes_firebase);

      // manage state so it is accessible by the commands
      app.manage(MyState::default());
      Ok(())
    })
    .build()
}
