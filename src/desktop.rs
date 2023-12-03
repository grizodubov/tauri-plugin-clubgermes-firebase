use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<ClubgermesFirebase<R>> {
  Ok(ClubgermesFirebase(app.clone()))
}

/// Access to the clubgermesfirebase APIs.
pub struct ClubgermesFirebase<R: Runtime>(AppHandle<R>);

impl<R: Runtime> ClubgermesFirebase<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
