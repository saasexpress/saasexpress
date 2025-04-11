import { Suspense, use, useState } from "react";
import { Box, Grid2, Stack } from "@mui/material";
import { UseQueryResult } from "@tanstack/react-query";
import useAPIClient, { GetResult } from "@lib/api/APIClient";
import Activity from "./Activity";

function ActivityList({
  query,
  handleChangePage,
  handleChangeRowsPerPage,
}: {
  query: UseQueryResult<GetResult, Error>;
  handleChangePage: any;
  handleChangeRowsPerPage: any;
}) {
  const results = use(query.promise);
  if (results.data.length === 0) {
    return <Box>No activity found</Box>;
  }
  const data = {
    items: results.data,
    paging: {
      totalPages: results.paging.total_pages,
      totalElements: results.paging.total_records,
      pageSize: results.paging.page_size,
      page: results.paging.current_page,
    },
  };
  return (
    <Activity
      data={data}
      handleChangePage={handleChangePage}
      handleChangeRowsPerPage={handleChangeRowsPerPage}
    />
  );
}

// function ListItems({ results }: { results: any }) {
//   return results.data.map((d: any) => <Item {...d} />);
// }

let ActivityController = () => {
  const api = useAPIClient();

  const [paging, setPaging] = useState<any>({
    totalPages: 0,
    totalElements: 0,
    pageSize: 5,
    page: 0,
  });

  const page = paging.page;
  const pageSize = paging.pageSize;

  const query = api.get(
    ["list-activity", paging],
    `/api/activity?page=${page}&recordsPerPage=${pageSize}`
  );

  const _handleActivity = (page: number, pageSize: number) => {
    setPaging({ ...paging, ...{ page, pageSize } });
  };

  return (
    <Grid2 container spacing={2}>
      <Grid2 size={{ md: 3, sm: 4, xs: 12 }}>
        <Stack bgcolor="white" p={4} mt={4} mb={0} borderRadius={0}>
          <Stack direction="column" alignItems="start" justifyContent="start">
            <Box>Searching..</Box>
          </Stack>
        </Stack>
      </Grid2>
      <Grid2 size={{ md: 9, sm: 8, xs: 12 }}>
        <Suspense fallback={<div>Loading...</div>}>
          <ActivityList
            query={query}
            handleChangePage={(e: any, page: number) => {
              _handleActivity(page, paging.pageSize);
            }}
            handleChangeRowsPerPage={(e: any) => {
              _handleActivity(paging.page, e.target.value);
            }}
          />
        </Suspense>
      </Grid2>
    </Grid2>
  );
};

export default ActivityController;
