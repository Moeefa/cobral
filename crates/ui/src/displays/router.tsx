import { Route, Routes } from "react-router-dom";

import { Changelog } from "@/displays/routes/changelog";
import { Docs } from "@/displays/routes/documentation";
import { Layout } from "@/displays/layouts/main";
import { Page } from "@/displays/routes/index";

export default function RoutesElement() {
  return (
    <Routes>
      <Route element={<Layout />}>
        <Route path="/" element={<Page />} />
        <Route path="/docs" element={<Docs />} />
        <Route path="/changelog" element={<Changelog />} />
      </Route>
    </Routes>
  );
}
