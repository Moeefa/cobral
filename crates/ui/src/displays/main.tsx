import "../styles/main.css";

import { App } from "./app";
import { EditorProvider } from "@/contexts/editor-context";
import ReactDOM from "react-dom/client";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <EditorProvider>
    <App />
  </EditorProvider>
);
