export const TAURI_PLATFORM = import.meta.env.TAURI_ENV_PLATFORM;
export const PLATFORM = TAURI_PLATFORM || 'web';
export const IS_TAURI = !!TAURI_PLATFORM;