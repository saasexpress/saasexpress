import { useEffect, useState, PropsWithChildren } from "react";
import { createContext, useContext } from "react";
import { useLocation, useSearchParams } from "react-router";

interface AppContextProps {
  env: { [key: string]: string };
  pathname: string | undefined;
  search?: URLSearchParams;
  config?: { [key: string]: any };
}

const AppContext = createContext<AppContextProps>({
  env: {},
  pathname: undefined,
});

export function AppContextProvider({ children }: PropsWithChildren) {
  const { pathname, search } = useLocation();
  const [ searchParams] =useSearchParams();
  

  const [env, setEnv] = useState({});
  const [config, _] = useState({});

  useEffect(() => {
    (async () => {
      //const response = await fetch("/api/runtime");
      //const runtime = await response.json();
      //setEnv(runtime);
    })();
  }, []);

  const sharedState = {
    pathname,
    search: searchParams,
    config,
    env,
  };

  return (
    <AppContext.Provider value={sharedState}>{children}</AppContext.Provider>
  );
}

export function useAppContext(): any {
  return useContext(AppContext);
}
