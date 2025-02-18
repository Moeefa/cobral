import { createContext, useCallback, useEffect, useState } from "react";

import type React from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";

interface SettingsContextProps {
	fontSize: number;
	setFontSize: (size: number | ((prev: number) => number)) => void;
}

export const SettingsContext = createContext({} as SettingsContextProps);

export function SettingsProvider({ children }: { children: React.ReactNode }) {
	const [fontSize, setFontSize] = useState(1);

	const setScale = useCallback(async () => {
		await getCurrentWebview().setZoom(fontSize);
	}, [fontSize]);

	useEffect(() => {
		setScale();
	}, [setScale]);

	return (
		<SettingsContext.Provider
			value={{
				fontSize,
				setFontSize,
			}}
		>
			{children}
		</SettingsContext.Provider>
	);
}
