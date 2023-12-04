use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "ru.clubgermes.social.plugin.firebase";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_clubgermes-firebase);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<ClubgermesFirebase<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "PushNotificationsPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_clubgermes-firebase)?;
  Ok(ClubgermesFirebase(handle))
}

/// Access to the clubgermes-firebase APIs.
pub struct ClubgermesFirebase<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> ClubgermesFirebase<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }
}
