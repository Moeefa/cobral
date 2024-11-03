import "../styles/main.css";

import { App } from "./app";
import { EditorProvider } from "@/contexts/editor-context";
import ReactDOM from "react-dom/client";
import { SettingsProvider } from "@/contexts/settings-context";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <SettingsProvider>
    <EditorProvider>
      <App />
    </EditorProvider>
  </SettingsProvider>
);
