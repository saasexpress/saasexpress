import * as React from "react";
import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";
import { Stack, Box, useMediaQuery, Theme } from "@mui/material";
import { Button } from "@mui/material";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import MoreVertIcon from "@mui/icons-material/MoreVert";
import styled from "@emotion/styled";
import { useTheme } from "@mui/material";

export default function GoldHorizontalTabs({ tabs, tab, onClick }: any) {
  const theme = useTheme();
  const isSmall = useMediaQuery(theme.breakpoints.down("sm"));

  const [value, setValue] = React.useState(0);

  React.useEffect(() => {
    setValue(
      tabs
        .map((tab: any, index: number) => ({ tab, index }))
        .filter((t: any) => t.tab.name === tab)
        .pop().index
    );
  }, tab);

  const handleChange = React.useCallback(
    (event: React.SyntheticEvent, tabIndex: number) => {
      setValue(tabIndex);
      onClick(tabs[tabIndex]);
    },
    []
  );

  const [anchorEl, setAnchorEl] = React.useState(null);
  const open = Boolean(anchorEl);
  const handleClick = (event: any) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };

  // const basePath = props.params?.id
  //   ? `/${props.collection}/${props.params?.id}`
  //   : `/${props.collection}`;

  return (
    <Box
      flexGrow={1}
      sx={{
        borderBottom: "1px solid #CCCCCC",
        // '&::after': {
        //   content: '""',
        //   display: 'inline-flex',
        //   position: 'relative',
        //   transform: 'translate(-50%, -50%)',
        //   translate: '0 -15px',
        //   // // top: 'calc(50% - 10px)',
        //   left: '50%',
        //   width: '100%',
        //   lineHeight: '10px',
        //   borderBottom: '4px solid red',
        //   zIndex: 10,
        // },
      }}
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
            {tabs.map((tab: any) => (
              <MenuItem>{tab.label}</MenuItem>
            ))}
          </Menu>
        </Stack>
      ) : (
        <StyledTabs
          value={value}
          onChange={handleChange}
          aria-label="styled tabs example"
        >
          {tabs.map((t: any) => (
            <StyledTab label={t.label}></StyledTab>
          ))}
        </StyledTabs>
      )}
    </Box>
    // <InlineAlert />
  );
}

// const LinkTab = React.forwardRef(({ href, ...rest }: any, ref: any) => (
//   <Link href={href} passHref ref={ref}>
//     <StyledTab {...rest} />
//   </Link>
// ));

const StyledTabs = styled((props: any) => (
  <Tabs
    {...props}
    TabIndicatorProps={{ children: <span className="MuiTabs-indicatorSpan" /> }}
  />
))({
  "& .MuiTabs-indicator": {
    display: "flex",
    height: "2px",
    justifyContent: "center",
    backgroundColor: "transparent",
  },
  "& .MuiTabs-indicatorSpan": {
    width: "90%",
    minWidth: "50px",
    backgroundColor: "rgb(252, 186, 25)",
  },
});

const TabTheme = ({ theme }: { theme?: Theme }) => ({
  fontWeight: theme?.typography.fontWeightRegular,
  fontSize: "1rem",
  marginRight: theme?.spacing(0),
  color: "rgba(0, 0, 0, 0.7)",

  "&:hover": {
    backgroundColor: "#eeeeee",
  },

  "&.Mui-selected": {
    color: "#000",
  },
  "&.Mui-focusVisible": {
    backgroundColor: "rgba(100, 95, 228, 0.32)",
  },
});

const StyledTab = styled(Tab)(TabTheme);
