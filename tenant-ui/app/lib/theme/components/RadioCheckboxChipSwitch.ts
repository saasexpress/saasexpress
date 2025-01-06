import type { Components, Theme } from "@mui/material";
import { red, pink, grey, deepPurple } from "@mui/material/colors";

const CustomTheme = {
  MuiAvatar: {
    styleOverrides: {
      root: {
        color: grey[200],
        backgroundColor: grey[400],
      },
    },
  },
  MuiRadio: {
    styleOverrides: {
      root: {
        width: 31,
        height: 31,
        padding: 0,
        margin: 0,
        color: "rgba(0, 0, 0, 0.54)",
        fontSize: "1rem",
      },
    },
  },
  MuiCheckbox: {
    styleOverrides: {
      root: {
        width: 31,
        height: 31,
        padding: 0,
        margin: 0,
        color: "rgba(0, 0, 0, 0.54)",
        cursor: "pointer",
        fontSize: "1rem",
      },
    },
  },
  MuiChip: {
    variants: [
      {
        props: { variant: "mui3" },
        style: {
          display: "flex",
          backgroundColor: grey[300], // 'rgb(232, 222, 248)',
          borderRadius: "8px",
          justifyContent: "center",
          fontWeight: 500,
          // flexWrap: 'wrap',
          "& > *": {
            margin: "0.5",
          },
        },
      },
    ],
    styleOverrides: {
      root: {
        borderRadius: "8px",
      },
      label: {
        // fontSize: '0.9rem',
      },
      avatarColorDefault: {
        color: grey[200],
        backgroundColor: grey[500],
      },
    },
  },
  MuiSwitch: {
    styleOverrides: {
      root: {
        width: 44,
        height: 22,
        padding: 0,
        margin: 0,
        display: "flex",
      },
      switchBase: {
        padding: 2,
        color: grey[500],
        transitionDuration: "300ms",
        "&.Mui-checked": {
          transform: "translateX(22px)",
          color: "white",
          "& + .MuiSwitch-track": {
            opacity: 1,
            backgroundColor: "#00688b",
            borderColor: "#00688b",
          },
        },
        "&.Mui-disabled": {
          color: "#CCCCCC",
          "& + .MuiSwitch-track": {
            opacity: 1,
            // backgroundColor: '#green',
            borderColor: "#CCCCCC",
          },
        },
      },
      thumb: {
        width: 16,
        height: 16,
        margin: 1,
        boxShadow: "none",
      },
      track: {
        border: `1px solid ${grey[500]}`,
        borderRadius: 44 / 2,
        opacity: 1,
        backgroundColor: "#FFFFFF",
      },

      checked: {},
    },
  },
} as Components<Theme>;

export const { MuiAvatar, MuiRadio, MuiCheckbox, MuiChip, MuiSwitch } =
  CustomTheme;
