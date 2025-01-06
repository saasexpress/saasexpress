import type { Components, Theme } from "@mui/material";
//import { red, pink, grey, deepPurple } from "@mui/material/colors";

const CustomTheme = {
  MuiAutocomplete: {
    styleOverrides: {
      // root: {
      //   padding: 0,
      // },
      inputRoot: {
        padding: 2.5,
      },
      endAdornment: {
        // top: 'calc(50% - 12px)',
      },
      // '&.MuiOutlinedInput': {
      //   root: {
      //     padding: 0,
      //   },
      // },
    },
  },
} as Components<Theme>;

export const { MuiAutocomplete } = CustomTheme;
