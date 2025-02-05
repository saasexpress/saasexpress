import { Container, Grid2 } from "@mui/material";
import { useParams } from "react-router";
import Header from "../header";
import Nav from "../nav";

import DetailHeader from "@components/detail-header";
import useAPIClient from "lib/api/APIClient";
import ServiceListController from "./service-list";

const Entry = () => {
  const { id } = useParams();

  const api = useAPIClient();

  if (!id) {
    return <></>;
  }

  const { isPending, data } = api.get(["tenant", id], `/api/tenants/${id}`, {
    refetchOnMount: true,
  });

  // if (isPending) {
  //   return <></>;
  // }

  const d = data?.data;

  const services = [
    {
      id: "1",
      name: "Service 1",
    },
  ];

  return (
    <Container maxWidth="lg">
      <Header name={d?.displayName} id={d?.id} />

      <Grid2
        key={d?.id}
        container
        direction="row"
        spacing={3}
        alignItems="top"
        mt={1}
      >
        <Grid2>
          <Nav tab="services" params={{ id: d?.id }} />
        </Grid2>
        <Grid2 flex={1}>
          <DetailHeader
            title="Services"
            description="Services available and protected by the tenant"
          />
          <ServiceListController />
        </Grid2>
      </Grid2>
    </Container>
  );
};

export default Entry;
