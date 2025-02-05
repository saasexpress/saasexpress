//import * as React from "react";
import { Box, Stack } from "@mui/material";
import Button from "@mui/material/Button";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import { styled } from "@mui/material/styles";
import Tab from "@mui/material/Tab";
import Tabs from "@mui/material/Tabs";
import { Link } from "react-router";

import MoreVertIcon from "@mui/icons-material/MoreVert";

import { useTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";
import { useState } from "react";

const LinkTab = ({ pathname, ...rest }: any) => (
  <Link to={{ pathname }}>
    <StyledTab {...rest} />
  </Link>
);

const StyledTabs = styled((props: any) => (
  <Tabs
    {...props}
    // TabIndicatorProps={{ children: <span className="MuiTabs-indicatorSpan" /> }}
  />
))({
  "& .MuiTabs-indicator": {
    display: "flex",
    height: "3px",
    backgroundColor: "transparent",
  },
  "& .MuiTabs-indicatorSpan": {
    width: "90%",
    minWidth: "50px",
    backgroundColor: "rgb(252, 186, 25)",
  },
});

const StyledTab = styled((props: any) => <Tab disableRipple {...props} />)(
  ({ theme }) => ({
    textTransform: "none",
    fontWeight: theme.typography.fontWeightRegular,
    fontSize: theme.typography.pxToRem(18),
    marginRight: theme.spacing(0),
    color: "rgba(0, 0, 0, 1)",
    textAlign: "left",
    minWidth: "200px",
    alignItems: "baseline",
    "&:hover": {
      backgroundColor: "#eeeeee",
    },
    "&.Mui-selected": {
      color: "white",
      backgroundColor: "rgb(0, 95, 115)",
      borderRadius: "3px",
    },
    "&.Mui-focusVisible": {
      color: "green",
      backgroundColor: "rgba(100, 95, 228, 0.32)",
    },
    "&.Mui-focusVisible:hover": {
      color: "green",
      backgroundColor: "rgba(100, 95, 228, 0.32)",
    },
  })
);

interface NavTabsProps {
  collection: string;
  tab: string;
  tabKeys: Record<string, string>;
  params?: Record<string, string>;
}

export default function NavTabs(props: NavTabsProps) {
  const tabKeys = props.tabKeys;

  const theme = useTheme();
  const isSmall = useMediaQuery(theme.breakpoints.down("sm"));

  const [value, setValue] = useState(Object.keys(tabKeys).indexOf(props.tab));

  const handleChange = (_: any, newValue: any) => {
    setValue(newValue);
  };

  const [anchorEl, setAnchorEl] = useState(null);
  const open = Boolean(anchorEl);
  const handleClick = (event: any) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };

  const basePath = props.params?.id
    ? `/${props.collection}/${props.params?.id}`
    : `/${props.collection}`;

  return (
    <Box p={0}>
      <Box
        sx={
          {
            // borderBottom: '1px solid #CCCCCC',
            // borderTop: '1px solid #CCCCCC',
          }
        }
      >
        {isSmall ? (
          <Stack direction="column" spacing={0} alignItems="flex-end">
            <Button
              id="basic-button"
              aria-controls={open ? "basic-menu" : undefined}
              aria-haspopup="true"
              aria-expanded={open ? "true" : undefined}
              onClick={handleClick}
            >
              <MoreVertIcon />
            </Button>
            ,
            <Menu
              id="basic-menu"
              anchorEl={anchorEl}
              open={open}
              onClose={handleClose}
              MenuListProps={{
                "aria-labelledby": "basic-button",
              }}
            >
              {Object.entries(tabKeys).map(([key, label], index) => (
                <Link key={index} to={{ pathname: `${basePath}/${key}` }}>
                  <MenuItem>{label}</MenuItem>
                </Link>
              ))}
            </Menu>
          </Stack>
        ) : (
          <StyledTabs
            orientation="vertical"
            value={value}
            onChange={handleChange}
            aria-label="styled tabs example"
          >
            {Object.entries(tabKeys).map(([key, label], index) => (
              <LinkTab
                key={index}
                pathname={`${basePath}/${key}`}
                label={label}
              ></LinkTab>
            ))}
          </StyledTabs>
        )}
      </Box>
    </Box>
  );
}
