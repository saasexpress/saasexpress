import React from "react";
import {
  Box,
  Divider,
  Drawer,
  IconButton,
  List,
  ListItemButton,
  ListItemText,
  AppBar as MuiAppBar,
  styled,
  Toolbar,
  Typography,
} from "@mui/material";
import { Link, Outlet } from "react-router";
import InlineAlert from "lib/alerts/InlineAlert";
import { useAppContext } from "context";

const drawerWidth = 240;

const Main = styled("main", { shouldForwardProp: (prop) => prop !== "open" })(
  ({ theme, open }: { theme?: any; open: boolean }) => ({
    flexGrow: 1,
    padding: theme.spacing(0),
    transition: theme.transitions.create("margin", {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.leavingScreen,
    }),
    marginTop: "0px",
    marginLeft: `-${drawerWidth}px`,
    ...(open && {
      transition: theme.transitions.create("margin", {
        easing: theme.transitions.easing.easeOut,
        duration: theme.transitions.duration.enteringScreen,
      }),
      marginLeft: 0,
    }),
  })
);

const AppBar = styled(MuiAppBar, {
  shouldForwardProp: (prop) => prop !== "open",
})(({ theme, open }: any) => ({
  transition: theme.transitions.create(["margin", "width"], {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  ...(open && {
    width: `calc(100% - ${drawerWidth}px)`,
    marginLeft: `${drawerWidth}px`,
    transition: theme.transitions.create(["margin", "width"], {
      easing: theme.transitions.easing.easeOut,
      duration: theme.transitions.duration.enteringScreen,
    }),
  }),
}));

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  padding: theme.spacing(0, 1),
  // necessary for content to be below app bar
  ...theme.mixins.toolbar,
  justifyContent: "flex-end",
}));

export default function AppLayout() {
  const [open] = React.useState(true);
  const { pathname } = useAppContext();

  return (
    <Box sx={{ display: "flex" }}>
      {/* <HideOnScroll {...props}> */}
      <AppBar color="info" elevation={0}>
        <Toolbar
          sx={{
            backgroundColor: "white",
            color: "black",
            borderBottom: "1px solid #CCCCCC",
            padding: 0,
            margin: 0,
          }}
        >
          <IconButton
            color="inherit"
            aria-label="open drawer"
            onClick={() => false}
            edge="start"
            sx={{ ml: 1, mr: 1, ...(open && { display: "none" }) }}
          >
            X
          </IconButton>
        </Toolbar>
      </AppBar>
      <Drawer
        PaperProps={{
          sx: { backgroundColor: "#2A3F54; rgb(10, 147, 150)", color: "white" },
        }}
        sx={{
          width: drawerWidth,
          flexShrink: 0,
          "& .MuiDrawer-paper": {
            width: drawerWidth,
            boxSizing: "border-box",
          },
        }}
        variant="persistent"
        anchor="left"
        open={open}
      >
        <DrawerHeader
          // variant="h2"
          sx={{ backgroundColor: "#223243; rgb(0, 48, 73)" }}
        >
          {/* <Link href="/" color="inherit"> */}
          <Typography
            variant="h2"
            noWrap
            component="div"
            sx={{
              flexGrow: 1,
              fontFamily: "Roboto Slab",
              fontSize: "26px",
            }}
          >
            SaaS Express
          </Typography>
        </DrawerHeader>
        <Divider />
        <List>
          {[
            { label: "Tenants", path: "/tenants" },
            { label: "Activity", path: "/activity" },
          ].map(({ label, path }) => (
            <Link to={path} key={label}>
              <ListItemButton
                selected={
                  (path === "/" && pathname === "/") ||
                  (path != "/" && pathname.startsWith(path))
                }
                // onClick={isSmall && handleDrawerClose}
                // className={
                //   appContext.pathname == path ? 'ListItemSelected' : ''
                // }
              >
                {/* {icon && <ListItemIcon>{icon}</ListItemIcon>} */}
                <ListItemText>{label}</ListItemText>
              </ListItemButton>
            </Link>
          ))}
        </List>
      </Drawer>
      <Main open={open}>
        <DrawerHeader />
        {/* <InlineAlertRedirect /> */}
        <InlineAlert />
        <Outlet />
      </Main>
    </Box>
  );
}
