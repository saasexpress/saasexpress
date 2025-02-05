import { Route, Routes } from "react-router";

import AppLayout from "layout";

import Another from "routes/another";
import Contact from "routes/contact";
import Home from "routes/home";
import Tenants from "routes/tenants";
import Shell from "./shell";
import ShellSettings from "./shell/[id]/settings";
import TenantProfile from "./routes/tenants/[id]/profile";
import TenantServices from "./routes/tenants/[id]/services";
import Activity from "./routes/activity";
import Services from "routes/services";
import ServiceDetails from "routes/services/[id]/details";
import ServiceEditor from "routes/services/[id]/editor";
import ServiceAPI from "routes/services/[id]/api";
import ServiceSchema from "routes/services/[id]/schema";

export default function AppRoutes() {
  return (
    <Routes>
      <Route element={<AppLayout />}>
        <Route index element={<Home />} />

        <Route path="/services" element={<Services />} />
        <Route path="/services/:id/details" element={<ServiceDetails />} />
        <Route path="/services/:id/editor" element={<ServiceEditor />} />
        <Route path="/services/:id/api" element={<ServiceAPI />} />
        <Route path="/services/:id/schema" element={<ServiceSchema />} />

        <Route path="/tenants" element={<Tenants />} />
        <Route path="/tenants/:id/profile" element={<TenantProfile />} />
        <Route path="/tenants/:id/services" element={<TenantServices />} />
        <Route path="/activity" element={<Activity />} />
        <Route path="/shell" element={<Shell />} />
        <Route path="/shell/:id/settings" element={<ShellSettings />} />
        <Route path="/contact" element={<Contact />} />
      </Route>
      <Route path="/another" element={<Another />} />
    </Routes>
  );
}
