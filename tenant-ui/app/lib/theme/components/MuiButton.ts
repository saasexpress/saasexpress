import type { Components, Theme } from "@mui/material";

const CustomTheme = {
  MuiButton: {
    variants: [
      {
        props: { variant: "condensed" },
        style: {
          minWidth: "1rem",
          borderRadius: "0.25rem",
          padding: 5,
          ":hover": {
            backgroundColor: "#EFEFEF",
          },
        },
      },
    ],
    styleOverrides: {
      root: {
        // fontSize: '1rem',
        fontWeight: 400,
        boxShadow: "none",
        disableElevation: true,
        minWidth: "4rem",
        textTransform: "none",
        // fontSize: '0.88rem',
      },
      contained: {
        borderRadius: "6rem",
      },
      outlined: {
        borderRadius: "6rem",
      },
      text: {
        borderRadius: "6rem",
        ":hover": {
          backgroundColor: "#EFEFEF",
        },
      },
    },
  },
  MuiButtonBase: {
    defaultProps: {
      disableRipple: true,
    },
  },
  MuiIconButton: {
    styleOverrides: {
      root: {
        minWidth: "1rem",
        borderRadius: "0.25rem",
        padding: 5,
        ":hover": {
          backgroundColor: "#EFEFEF",
        },
      },
    },
  },
} as Components<Theme>;

export const { MuiButton, MuiIconButton, MuiButtonBase } = CustomTheme;
