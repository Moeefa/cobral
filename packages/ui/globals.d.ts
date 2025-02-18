declare module "*.worker?worker";
declare module "*.md";

declare global {
	interface Window {
		__TAURI__: unknown;
	}
}

window.__TAURI__ = window.__TAURI__ || {};
