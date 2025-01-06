import { Container, Grid2 } from "@mui/material";
import { useParams } from "react-router";
import Header from "../header";
import Nav from "../nav";

import DetailHeader from "@components/detail-header";
import SettingsForm from "./settings-form";

interface SettingsEntryProps {}

const SettingsEntry = (params: SettingsEntryProps) => {
  const { id } = useParams();
  return (
    id && (
      <Container maxWidth="lg">
        <Header />
        <Grid2 container direction="row" spacing={3} alignItems="top" mt={1}>
          <Grid2>
            <Nav tab="settings" params={{ id }} />
          </Grid2>
          <Grid2 flex={1}>
            <DetailHeader title="Settings" description="Shell settings" />
            <SettingsForm id={id} />
          </Grid2>
        </Grid2>
      </Container>
    )
  );
};

export default SettingsEntry;
