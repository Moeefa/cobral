import { Route, Routes } from "react-router-dom";

import { Changelog } from "./routes/changelog";
import { Docs } from "@/displays/routes/documentation";
import { Page } from "./routes/index";
import { Titlebar } from "@/components/titlebar";

export default function RoutesElement() {
  return (
    <Routes>
      <Route element={<Titlebar />}>
        <Route path="/" element={<Page />} />
        <Route path="/docs" element={<Docs />} />
        <Route path="/changelog" element={<Changelog />} />
      </Route>
    </Routes>
  );
}
