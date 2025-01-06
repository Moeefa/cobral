import { createContext, useEffect, useState } from "react";

import React from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";

interface SettingsContextProps {
  fontSize: number;
  setFontSize: (size: number | ((prev: number) => number)) => void;
}

export const SettingsContext = createContext({} as SettingsContextProps);

export function SettingsProvider({ children }: { children: React.ReactNode }) {
  const [fontSize, setFontSize] = useState(1);

  async function setScale() {
    await getCurrentWebview().setZoom(fontSize);
  }

  useEffect(() => {
    setScale();
  }, [fontSize]);

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
