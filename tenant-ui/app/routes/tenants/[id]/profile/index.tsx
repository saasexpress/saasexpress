import { Container, Grid2 } from "@mui/material";
import { useParams } from "react-router";
import Header from "../header";
import Nav from "../nav";

import DetailHeader from "@components/detail-header";
import SettingsForm from "./profile-form";
import useAPIClient from "lib/api/APIClient";

const Entry = () => {
  const { id } = useParams();

  const api = useAPIClient();

  if (!id) {
    return <></>;
  }

  const { isPending, data } = api.get(["tenant", id], `/api/tenants/${id}`, {
    refetchOnMount: true,
  });

  if (isPending) {
    return <></>;
  }

  const d = data?.data;

  return (
    <Container maxWidth="lg">
      <Header name={d.displayName} id={d.id} />
      <Grid2
        key={d.id}
        container
        direction="row"
        spacing={3}
        alignItems="top"
        mt={1}
      >
        <Grid2>
          <Nav tab="profile" params={{ id: d.id }} />
        </Grid2>
        <Grid2 flex={1}>
          <DetailHeader title="Profile" description="Tenant profile" />
          <SettingsForm item={d} />
        </Grid2>
      </Grid2>
    </Container>
  );
};

export default Entry;
