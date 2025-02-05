import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router";
import { Provider } from "./provider";
import enableMocking from "./mocks/init";
import AppRoutes from "routes";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AppContextProvider } from "context";

import "./app.css";

// Create a query client and set it up to be used in the app
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      experimental_prefetchInRender: true,
      refetchOnWindowFocus: true,
    },
  },
});

enableMocking().then(() => {
  createRoot(document.getElementById("root")!).render(
    <StrictMode>
      <QueryClientProvider client={queryClient}>
        <BrowserRouter basename="ui">
          <AppContextProvider>
            <Provider>
              <AppRoutes />
            </Provider>
          </AppContextProvider>
        </BrowserRouter>
      </QueryClientProvider>
    </StrictMode>
  );
});
