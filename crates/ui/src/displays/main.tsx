import "../styles/main.css";

import { App } from "./app";
import ReactDOM from "react-dom/client";
import { Titlebar } from "@/components/titlebar";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <>
    <Titlebar />
    <App />
  </>
);
