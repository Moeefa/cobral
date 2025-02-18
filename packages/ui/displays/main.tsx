import "../styles/main.css";

import { App } from "./app";
import { EditorProvider } from "@/contexts/editor-context";
import ReactDOM from "react-dom/client";
import { SettingsProvider } from "@/contexts/settings-context";
import { Toaster } from "@/components/ui/toaster";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<SettingsProvider>
		<EditorProvider>
			<App />
			<Toaster />
		</EditorProvider>
	</SettingsProvider>,
);
