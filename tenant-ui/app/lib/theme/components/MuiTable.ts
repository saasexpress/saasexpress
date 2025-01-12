import type { Components, Theme } from "@mui/material";
import red from "@mui/material/colors/red";
import grey from "@mui/material/colors/grey";

const CustomTheme = {
  MuiTableCell: {
    styleOverrides: {
      root: {
        padding: 16,
        fontSize: "1rem",
        "&.MuiTableCell-body": {},
        "&.MuiTableCell-head": {
          fontSize: "0.8rem",
          fontWeight: 600,
          color: "#005f73",
          textTransform: "uppercase",
          // textDecoration: "overline red",
        },
      },
    },
  },
  MuiTablePagination: {
    styleOverrides: {
      root: {
        fontSize: "1rem",
      },
      selectLabel: {
        fontSize: "1rem",
      },
      select: {
        fontSize: "1rem",
      },
      displayedRows: {
        fontSize: "1rem",
      },
    },
  },
} as Components<Theme>;

export const { MuiTableCell, MuiTablePagination } = CustomTheme;
