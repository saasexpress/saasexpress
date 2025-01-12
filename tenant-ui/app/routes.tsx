import { Route, Routes } from "react-router";

import AppLayout from "layout";

import Another from "routes/another";
import Contact from "routes/contact";
import Home from "routes/home";
import Tenants from "routes/tenants";
import Shell from "./shell";
import ShellSettings from "./shell/[id]/settings";
import TenantProfile from "./routes/tenants/[id]/profile";
import Activity from "./routes/activity";

export default function AppRoutes() {
  return (
    <Routes>
      <Route element={<AppLayout />}>
        <Route index element={<Home />} />
        <Route path="/tenants" element={<Tenants />} />
        <Route path="/tenants/:id/profile" element={<TenantProfile />} />
        <Route path="/activity" element={<Activity />} />
        <Route path="/shell" element={<Shell />} />
        <Route path="/shell/:id/settings" element={<ShellSettings />} />
        <Route path="/contact" element={<Contact />} />
      </Route>
      <Route path="/another" element={<Another />} />
    </Routes>
  );
}
