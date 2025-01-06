import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router";
import { Provider } from "./provider";
import enableMocking from "./mocks/init";
import AppRoutes from "routes";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AppContextProvider } from "context";
import InlineAlert from "lib/alerts/InlineAlert";

// Create a query client and set it up to be used in the app
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      experimental_prefetchInRender: true,
    },
  },
});

enableMocking().then(() => {
  createRoot(document.getElementById("root")!).render(
    <StrictMode>
      <BrowserRouter basename="ui">
        <AppContextProvider>
          <QueryClientProvider client={queryClient}>
            <Provider>
              <AppRoutes />
            </Provider>
          </QueryClientProvider>
        </AppContextProvider>
      </BrowserRouter>
    </StrictMode>
  );
});
