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

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the clubgermesfirebase APIs.
pub trait ClubgermesFirebaseExt<R: Runtime> {
  fn clubgermesfirebase(&self) -> &ClubgermesFirebase<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ClubgermesFirebaseExt<R> for T {
  fn clubgermesfirebase(&self) -> &ClubgermesFirebase<R> {
    self.state::<ClubgermesFirebase<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("clubgermesfirebase")
    .invoke_handler(tauri::generate_handler![commands::execute])
    .setup(|app, api| {
      #[cfg(mobile)]
      let clubgermesfirebase = mobile::init(app, api)?;
      #[cfg(desktop)]
      let clubgermesfirebase = desktop::init(app, api)?;
      app.manage(clubgermesfirebase);

      // manage state so it is accessible by the commands
      app.manage(MyState::default());
      Ok(())
    })
    .build()
}
