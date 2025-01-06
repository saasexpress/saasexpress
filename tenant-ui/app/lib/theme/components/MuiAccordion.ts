import type { Components, Theme } from "@mui/material";

const CustomTheme = {
  MuiAccordionSummary: {
    styleOverrides: {
      content: {
        "&.Mui-expanded": {
          // marginLeft: '8px',
        },
        ".MuiTypography-root": {},
      },
      expandIconWrapper: {
        ".MuiSvgIcon-root": {
          fontSize: "0.875rem",
        },
      },
    },
  },
} as Components<Theme>;

export const { MuiAccordionSummary } = CustomTheme;
