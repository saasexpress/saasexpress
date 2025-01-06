import React from "react";
import { Box, Grid2 } from "@mui/material";
import ShellItem from "./item";
import useAPIClient, { GetResult } from "@lib/api/APIClient";

import { UseQueryResult } from "@tanstack/react-query";

function ShellListItems({
  query,
}: {
  query: UseQueryResult<GetResult, Error>;
}) {
  const results = React.use(query.promise);

  return (
    <Box>
      {results.data.map((d: any) => (
        <ShellItem key={d._id} {...d} />
      ))}
    </Box>
  );
}

export default function ShellList() {
  const api = useAPIClient();

  const query = api.get(["list-tenants"], `/api/tenants`, {
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
        <ShellListItems query={query}></ShellListItems>
      </React.Suspense>
    </Grid2>
  );
}
