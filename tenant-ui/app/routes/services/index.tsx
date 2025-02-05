import LayersIcon from "@mui/icons-material/Layers";
import { Container, Stack, Grid2, Typography } from "@mui/material";
import List from "./list-controller";
import NewDialog from "./new";
import { useQueryClient } from "@tanstack/react-query";
import APIErrorHandler from "lib/alerts/APIErrorHandler";
import { useNavigate } from "react-router";

const queryKey = ["service"];

export default function Page() {
  const queryClient = useQueryClient();
  const navigate = useNavigate();

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
            <LayersIcon fontSize="large" />
            <Typography paddingLeft={1} variant="h3">
              List of Services
            </Typography>
          </Stack>
          <Typography variant="subtitle1" sx={{ mt: 0.5 }}>
            Services allow you to use integration patterns and create custom
            APIs
          </Typography>
        </Grid2>
        <Grid2 flex={1} textAlign="right">
          <NewDialog
            onCreated={(d: any) => {
              queryClient.invalidateQueries({ queryKey });
              queryClient.invalidateQueries({ queryKey: ["list-services"] });

              navigate(`/services/${d.id}/editor`);
              APIErrorHandler.notice({
                title: "Service",
                content: "Created successfully.",
              });
            }}
          />
        </Grid2>
      </Grid2>
      <List />
    </Container>
  );
}
