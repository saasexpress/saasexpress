import NavTabs from "@lib/navtabs";

const collection = "services";

const tabKeys = {
  // details: "Profile",
  editor: "Integration",
  api: "API",
  schema: "Schema",
};

interface NavigationProps {
  tab: string;
  params: { id: string };
}

export default function Navigation({ tab, params }: NavigationProps) {
  return (
    <NavTabs
      collection={collection}
      tabKeys={tabKeys}
      tab={tab}
      params={params}
    />
  );
}
