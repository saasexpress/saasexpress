import NavTabs from "@lib/navtabs";

const collection = "tenants";

const tabKeys = {
  profile: "Profile",
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
