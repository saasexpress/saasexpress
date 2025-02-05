import { Typography, Container, Grid2, Stack, Icon } from "@mui/material";
import ActivityController from "components/activity/activity-controller";

import HistoryIcon from "@mui/icons-material/History";
//const ActivityIcon = <Icon baseClassName="fa" className="fa-stream" />;

export default function ActivityPage() {
  return (
    <Container maxWidth="lg">
      <Grid2
        container
        p={4}
        sx={{ backgroundColor: "rgb(237, 237, 237)" }}
        mt={4}
      >
        <Grid2>
          <Stack direction="row" alignItems="center" justifyContent="left">
            <HistoryIcon fontSize="large" />
            <Typography paddingLeft={1} variant="h3">
              Activity
            </Typography>
          </Stack>
          <Typography variant="subtitle1" sx={{ mt: 0.5 }}>
            List all Tenant Activity
          </Typography>
        </Grid2>
      </Grid2>

      <ActivityController />
    </Container>
  );
}
