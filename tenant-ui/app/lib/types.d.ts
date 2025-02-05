import {
  Palette as MuiPallete,
  PaletteOptions as MuiPaletteOptions,
} from "@mui/material/styles/createPalette";

declare module "./createEmotionCache" {
  export default function createEmotionCache(): any;
}

declare module "@mui/material/styles/createPalette" {
  interface Palette extends MuiPallete {
    header: { main: string; contrastText: string };
  }

  interface PaletteOptions {
    header?: { main: string; contrastText: string };
  }
}

declare module "@mui/material/styles" {
  interface TypographyVariants {
    link: any;
  }

  // allow configuration using `createTheme`
  interface TypographyVariantsOptions {
    link?: any;
  }
}

declare module "@mui/material/Typography" {
  interface TypographyPropsVariantOverrides {
    link: true;
  }
}

declare module "@mui/material/Button" {
  interface ButtonPropsVariantOverrides {
    condensed: true;
  }
}

declare module "@mui/material/Chip" {
  interface ChipPropsVariantOverrides {
    mui3: true;
  }
}

declare module "@mui/material/Typography" {
  interface TypographyPropsVariantOverrides {
    link: true;
  }
}

declare module "@mui/material/ListItemButton" {
  interface ListItemButtonPropsVariantOverrides {
    dark: true;
  }
}
