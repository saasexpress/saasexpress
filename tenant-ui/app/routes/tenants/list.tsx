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
  return results.data.map((d: any) => <Item {...d} />);
}

export default function List() {
  const api = useAPIClient();

  const query = api.get(queryKey, `/api/tenants`, {
    refetchOnMount: true,
  });

  return (
    <Grid2
      container
      direction="row"
      spacing={2}
      mt={2}
      mb={2}
      alignItems="stretch"
    >
      <React.Suspense fallback={<div>Loading...</div>}>
        <CheckEmptyList query={query} />
      </React.Suspense>
    </Grid2>
  );
}
