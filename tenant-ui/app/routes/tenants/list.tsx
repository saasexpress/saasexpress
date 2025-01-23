import React from "react";
import { Box, Grid2 } from "@mui/material";
import Item from "./item";
import useAPIClient, { GetResult } from "@lib/api/APIClient";

import { UseQueryResult } from "@tanstack/react-query";

export const queryKey = ["list-tenants"];

function CheckEmptyList({
  query,
}: {
  query: UseQueryResult<GetResult, Error>;
}) {
  const results = React.use(query.promise);
  if (results.data.length === 0) {
    return <Box>No tenants found</Box>;
  }
  return <ListItems results={results} />;
}

function ListItems({ results }: { results: any }) {
  return results.data.map((d: any) => <Item key={d.id} {...d} />);
}

export default function List() {
  const api = useAPIClient();

  const query = api.get(queryKey, `/api/tenants`, {
    refetchOnMount: true,
  });

  return (
    <React.Suspense fallback={<div>Loading...</div>}>
      <Grid2
        data-id="list-tenants"
        container
        direction="row"
        spacing={2}
        mt={2}
        mb={2}
        alignItems="stretch"
      >
        <CheckEmptyList query={query} />
      </Grid2>
    </React.Suspense>
  );
}
