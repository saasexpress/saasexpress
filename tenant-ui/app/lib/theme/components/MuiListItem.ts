import type { Components, Theme } from "@mui/material";

const CustomTheme = {
  MuiListItemIcon: {
    styleOverrides: {
      root: {
        color: "white",
        minWidth: "2.5rem",
      },
    },
  },
  MuiListItemText: {
    styleOverrides: {
      primary: {
        color: "white",
        // fontSize: '80%',
        // textTransform: 'uppercase',
      },
    },
  },
  MuiListItemButton: {
    styleOverrides: {
      root: {
        ":hover": {
          backgroundColor: "#005f73",
        },
        "&.Mui-selected": {
          backgroundColor: "#334c65",
        },
        "&.Mui-selected:hover": {
          backgroundColor: "#334c65",
        },
      },
    },
  },
} as Components<Theme>;

export const { MuiListItemIcon, MuiListItemText, MuiListItemButton } =
  CustomTheme;
