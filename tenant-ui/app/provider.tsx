import { ReactNode } from "react";
import { ThemeProvider, CssBaseline } from "@mui/material";
import { CacheProvider } from "@emotion/react";
import { IntlProvider } from "react-intl";

import theme from "@lib/theme";
import createEmotionCache from "@lib/createEmotionCache";
//import { AppContextProvider } from "context";

// Client-side cache, shared for the whole session of the user in the browser.
const clientSideEmotionCache = createEmotionCache();

export function Provider({ children }: { children: ReactNode }) {
  return (
    <CacheProvider value={clientSideEmotionCache}>
      <IntlProvider locale="en">
        <ThemeProvider theme={theme}>
          <CssBaseline />
          {children}
        </ThemeProvider>
      </IntlProvider>
    </CacheProvider>
  );
}
