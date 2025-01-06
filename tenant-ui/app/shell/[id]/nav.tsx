import React from "react";
import NavTabs from "@lib/navtabs";

const tabKeys = {
  blocks: "Building Blocks",
  variants: "Variants",
  activity: "Activity",
  settings: "Settings",
};

interface ShellNavigationProps {
  tab: string;
  params: { id: string };
}

export default function ShellNavigation({ tab, params }: ShellNavigationProps) {
  return (
    <NavTabs collection="shell" tabKeys={tabKeys} tab={tab} params={params} />
  );
}
