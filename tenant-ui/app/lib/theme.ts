import { createTheme } from "@mui/material";
import { red } from "@mui/material/colors";

import {
  MuiButton,
  MuiButtonBase,
  MuiIconButton,
} from "./theme/components/MuiButton";

import {
  MuiListItemIcon,
  MuiListItemText,
  MuiListItemButton,
} from "./theme/components/MuiListItem";

import {
  MuiAvatar,
  MuiRadio,
  MuiCheckbox,
  MuiChip,
  MuiSwitch,
} from "./theme/components/RadioCheckboxChipSwitch";

import { MuiAccordionSummary } from "./theme/components/MuiAccordion";

import { MuiAutocomplete } from "./theme/components/MuiAutocomplete";

import { MuiTableCell, MuiTablePagination } from "./theme/components/MuiTable";

// const salmonBase = '#FF5733';
// const salmonMain = alpha(salmonBase, 0.7);

// Montserrat

const theme = createTheme({
  typography: {
    fontFamily:
      "Robo Condensed, -apple-system, system-ui, BlinkMacSystemFont, 'Segoe UI', Roboto, Ubuntu",
    body1: {},
    link: {
      color: "rgb(14, 112, 190)",
      ":hover": {
        textDecoration: "underline",
      },
    },
    h1: {
      fontFamily: "Roboto Slab",
      fontSize: "2.25rem",
    },
    h2: {
      fontFamily: "Roboto Slab",
      fontSize: "2rem",
    },
    h3: {
      fontSize: "1.70rem",
      fontWeight: 600,
    },
    h4: {
      fontSize: "1.35rem",
      fontWeight: 600,
    },
    h5: {
      fontSize: "1rem",
      fontWeight: 600,
    },
    h6: {
      fontSize: "1rem",
      fontWeight: 600,
    },
  },
  palette: {
    action: {
      hoverOpacity: 0,
    },
    primary: {
      main: "#005f73",
    },
    secondary: {
      main: "#0a9396",
    },
    header: {
      main: "#003049",
      contrastText: "#FFFFFF",
    },
    error: {
      main: red.A400,
    },
  },
  components: {
    MuiAccordionSummary,
    MuiAutocomplete,
    MuiAvatar,
    MuiButton,
    MuiButtonBase,
    MuiCheckbox,
    MuiChip,
    MuiIconButton,
    MuiListItemIcon,
    MuiListItemText,
    MuiListItemButton,
    MuiRadio,
    MuiSwitch,
    MuiTableCell,
    MuiTablePagination,
    MuiCssBaseline: {
      styleOverrides: {
        html: {
          fontSize: "16px",
        },
        body: {
          //backgroundColor: '#f2f5fb', Light blue
          backgroundColor: "rgb(249, 249, 249)",
        },
        a: {
          textDecoration: "none",
          color: "black",
        },
      },
    },
    // MuiTypography: {
    //   defaultProps: {
    //     variantMapping: {
    //       // Map the new variant to render a <h1> by default
    //       link: 'h1',
    //     },
    //   },
    // },
    MuiPaper: {
      styleOverrides: {
        outlined: {
          borderRadius: 0,
          borderWidth: 0,
        },
      },
    },
    MuiIcon: {
      defaultProps: {
        baseClassName: "material-icons",
      },
    },
    MuiInputBase: {
      styleOverrides: {
        input: {
          // fontSize: '1rem',
          color: "black",
          // backgroundColor: '#EFEFEF',
          // '&.MuiOutlinedInput-input': {
          //   padding: 0,
          // },
        },
        // textarea: {
        //   padding: 0,
        // },
        multiline: {
          padding: "0 !important",
        },
      },
    },
    MuiOutlinedInput: {
      styleOverrides: {
        input: {
          padding: 10,
        },
        // noborder: {
        //   border: 0,
        // },
        // root: {
        //   '&.MuiAutocomplete': {
        //     input: {
        //       padding: 0,
        //     },
        //   },
        //   '&.MuiAutocomplete-input': {
        //     padding: 0,
        //   },
        // },
      },
    },
  },
});

export default theme;
