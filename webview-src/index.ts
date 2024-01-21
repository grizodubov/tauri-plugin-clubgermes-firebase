import { invoke } from '@tauri-apps/api/core'

export async function execute() {
  await invoke('plugin:clubgermes-firebase|execute')
}
<<<<<<< HEAD
=======

export async function getMyToken(): Promise<string[] | null> {
  return await invoke<string[] | null>("plugin:clubgermes-firebase|getMyToken");
}
>>>>>>> 273fbb6 (second ;-) commit)
