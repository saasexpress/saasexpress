import type { Components, Theme } from "@mui/material";
import red from "@mui/material/colors/red";
import grey from "@mui/material/colors/grey";

const CustomTheme = {
  MuiTableCell: {
    styleOverrides: {
      root: {
        color: red[900],
        // fontSize: '1rem',
        padding: 16,
        "&.MuiTableCell-head": {
          color: grey[900],
          textTransform: "uppercase",
          //textDecoration: 'overline red',
        },
      },
    },
  },
} as Components<Theme>;

export const { MuiTableCell } = CustomTheme;
