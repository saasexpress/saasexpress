import React from "react";
import { Link } from "react-router";
import { Box, Grid, Stack, Typography } from "@mui/material";
import MoreVertIcon from "@mui/icons-material/MoreVert";
import VisibilityIcon from "@mui/icons-material/Visibility";
import NotificationDialog from "@components/gold/notification-dialog";
// import NotifyEntry from "@/components/_blueprint/components/notify/NotifyEntry";
// import { useAppContext } from "@/lib/context";
// import useAPIClientTanstack from "@/components/_common/APIClientTanstack";

interface HeaderProps {
  name?: string;
}

export default function Header({ name }: HeaderProps) {
  // const {
  //   query: { id },
  // } = useAppContext();

  // const api = useAPIClientTanstack();

  const summary = { data: { label: "label" } };
  // const { data: summary } = api.get(
  //   ["blueprint", id],
  //   `/api/v2/blueprints/${id}`
  // );

  return (
    <Grid
      container
      direction="row"
      alignItems="top"
      p={4}
      mt={4}
      sx={{ backgroundColor: "rgb(237, 237, 237)" }}
    >
      <Grid item>
        <Stack direction="row" alignItems="center" justifyContent="left" mb={1}>
          <Typography variant="h3" paddingLeft={0}>
            Shell {summary?.data?.label}
          </Typography>
        </Stack>
        <Link to={{ pathname: "/shell" }}>
          <Typography variant="link">List of Shells</Typography>
        </Link>
      </Grid>
      <Grid item flex={1} textAlign="right">
        <Stack direction="row" spacing={1} justifyContent="flex-end">
          <Link to="">
            <VisibilityIcon fontSize="large" sx={{ fill: "#999999" }} />
          </Link>
          <NotificationDialog
            title={"My Notification Preferences"}
            description={`Shell ${summary?.data?.label}`}
          >
            <Box>blah</Box>
            {/* <NotifyEntry params={{ id }} /> */}
          </NotificationDialog>
          <Link to="">
            <MoreVertIcon fontSize="large" sx={{ fill: "#999999" }} />
          </Link>
        </Stack>
      </Grid>
    </Grid>
  );
}
