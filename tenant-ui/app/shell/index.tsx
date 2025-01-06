import LayersIcon from "@mui/icons-material/Layers";
import { Container, Stack, Grid2, Typography } from "@mui/material";
import ShellList from "./list";
// import NewBlueprintDialog from "./new";

export default function ShellPage() {
  const actions = false; //<NewBlueprintDialog onCreated={() => {}} />;

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
            <LayersIcon />
            <Typography paddingLeft={1} variant="h3">
              List of Shells
            </Typography>
          </Stack>
          <Typography variant="subtitle1" sx={{ mt: 0.5 }}>
            Shells is a sample entity
          </Typography>
        </Grid2>
        <Grid2 flex={1} textAlign="right">
          {actions}
        </Grid2>
      </Grid2>
      <ShellList />
    </Container>
  );
}
