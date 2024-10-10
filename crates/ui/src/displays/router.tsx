import { Route, Routes } from "react-router-dom";

import { Page } from "./routes/index";

export default function RoutesElement() {
  return (
    <Routes>
      <Route path="/" element={<Page />} />
    </Routes>
  );
}
