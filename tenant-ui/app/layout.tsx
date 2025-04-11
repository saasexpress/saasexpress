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
  Stack,
  styled,
  Toolbar as MuiToolbar,
  Typography,
} from "@mui/material";
import { Link, Outlet } from "react-router";
import InlineAlert from "lib/alerts/InlineAlert";
import { useAppContext } from "context";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import MenuIcon from "@mui/icons-material/Menu";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";

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

const Toolbar = styled(MuiToolbar, {
  shouldForwardProp: (prop) => prop !== "open",
})(({ theme, open }: { theme?: any; open: boolean }) => ({
  ...{ margin: 0 },
  ...(open && {
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
  const [open, setOpen] = React.useState(false);
  const { pathname } = useAppContext();

  return (
    <Box sx={{ display: "flex" }}>
      {/* <HideOnScroll {...props}> */}
      <AppBar color="transparent" elevation={0}>
        <Toolbar
          open={open}
          sx={{
            backgroundColor: "white",
            color: "black",
            borderBottom: "1px solid #CCCCCC",
            padding: 0,
            verticalAlign: "middle",
          }}
        >
          {/* <IconButton
            color="inherit"
            aria-label="open drawer"
            onClick={() => setOpen(true)}
            edge="start"
            sx={{ ml: 1, mr: 1, ...(open && { display: "none" }) }}
          >
            ...
          </IconButton> */}
          {!open && (
            <IconButton sx={{ mt: 0.5, mr: 0 }} onClick={() => setOpen(true)}>
              <MenuIcon sx={{ color: "black" }} />
            </IconButton>
          )}

          <Stack
            direction="row"
            ml={2}
            alignItems="center"
            sx={{
              flexGrow: 1,
            }}
          >
            {[
              { label: "Tenants", path: "/tenants" },
              { label: "Services", path: "/services" },
              { label: "Activity", path: "/activity" },
            ].map(({ label, path }, index: number) => [
              index > 0 && <Typography pl={0} pr={0}></Typography>,
              <Link to={path} key={label}>
                <ListItemButton
                  sx={{ mr: 1, p: 1 }}
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
                  <ExpandMoreIcon />
                </ListItemButton>
              </Link>,
            ])}
          </Stack>
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
          <IconButton
            onClick={() => setOpen(false)}
            sx={{
              ":hover": {
                backgroundColor: "rgb(0, 95, 115)",
              },
            }}
          >
            <ChevronLeftIcon sx={{ color: "white" }} />
          </IconButton>
        </DrawerHeader>
        <Divider />
        <List>
          {[
            { label: "Tenants", path: "/tenants" },
            { label: "Services", path: "/services" },
            { label: "Activity", path: "/activity" },
          ].map(({ label, path }) => (
            <Link to={path} key={label}>
              <ListItemButton
                sx={{
                  color: "white",
                  ":hover": {
                    backgroundColor: "rgb(0, 95, 115)",
                  },
                  "&.Mui-selected": {
                    backgroundColor: "#334c65",
                  },
                  "&.Mui-selected:hover": {
                    backgroundColor: "#334c65",
                  },
                }}
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
