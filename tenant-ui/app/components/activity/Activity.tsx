import { useMemo } from "react";
import TimeAgo from "react-timeago";
import {
  Box,
  Grid2,
  IconButton,
  Paper,
  Stack,
  Table,
  TableBody,
  TableContainer,
  TableHead,
  TablePagination,
  TableRow,
  Typography,
} from "@mui/material";
import groupBy from "lodash/groupBy";
import RefreshIcon from "@mui/icons-material/Refresh";
import TableCell from "components/table-cell";
import CustomizedTooltip from "components/gold/tooltip-simple";
import { useCallback } from "react";
import { useQueryClient } from "@tanstack/react-query";
import ActivityItem, {
  ActivitySortDate,
  ActivitySummary,
  uid,
} from "./ActivityItem";

// const StyledPaper = styled(Paper)(() => ({
//   boxShadow: "none",
// }));

const timeZone = "America/Vancouver";

const sortFormat = new Intl.DateTimeFormat("en-CA", {
  dateStyle: "short",
  timeZone,
});

const headerFormat = new Intl.DateTimeFormat("en-CA", {
  dateStyle: "long",
  timeZone,
});

let ActivityItemRow = (a: any) => {
  // const { changes, version, count } = renderChanges(a.activity);

  let ts = <TimeAgo date={new Date(a.activityAt)} />;

  return (
    <TableRow
      key={a.id}
      sx={{
        "&:last-child td, &:last-child th": { border: 0 },
        verticalAlign: "top",
      }}
    >
      <TableCell>{a.message}</TableCell>
      <TableCell>{JSON.stringify(a.params)}</TableCell>
      <TableCell>{a.result}</TableCell>
      <TableCell sx={{ whiteSpace: "nowrap" }}>
        <CustomizedTooltip placement="top" tooltip={a.activityAt}>
          {ts}
        </CustomizedTooltip>
      </TableCell>
    </TableRow>
  );
};

let Activity = ({ data, handleChangePage, handleChangeRowsPerPage }: any) => {
  const { items, paging } = data;

  const query = useQueryClient();

  const doRefresh = useCallback(
    (_: any) => {
      query.invalidateQueries({ queryKey: ["list-activity"] });
    },
    [paging]
  );

  const feed: Record<string, ActivitySortDate[]> = useMemo(() => {
    let result = {};
    result = items
      // .reduce((memo: any, page: any) => {
      //   return memo.concat(page.getFilteredNamespaceActivity);
      // }, [])
      .filter((a: ActivitySummary) => a.activityAt)
      .map((a: ActivitySummary) => {
        const sortDate = new Date(a.activityAt);
        return { ...a, sortDate: sortFormat.format(sortDate) };
      });
    return groupBy(result, "sortDate");
  }, [items]);

  const handleParamSelect = (key: string, value: string) => {
    // addFilter(key, { value: value, name: value, multiple: true });
  };

  return (
    <Stack bgcolor="white" p={4} mt={4} mb={4} borderRadius={0}>
      <Stack direction="row" alignItems="center" justifyContent="right">
        <TablePagination
          width="100%"
          rowsPerPageOptions={[5, 10, 25]}
          component={Box}
          count={paging.totalElements}
          rowsPerPage={paging.pageSize}
          page={paging.page}
          onPageChange={handleChangePage}
          onRowsPerPageChange={handleChangeRowsPerPage}
        />
        <Grid2 flexGrow={1} textAlign="right">
          <IconButton onClick={doRefresh}>
            <RefreshIcon />
          </IconButton>
        </Grid2>
      </Stack>

      {Object.keys(feed).map((date) => {
        return (
          <Box key={uid(date)}>
            <Typography
              variant="h6"
              component="h2"
              mb={4}
              data-testid={`activity-feed-heading-${date}`}
            >
              {headerFormat.format(new Date(date.replaceAll("-", "/")))}
            </Typography>
            {feed[date].map((a) => (
              <ActivityItem
                key={uid(a.id)}
                data={a}
                onSelect={handleParamSelect}
              />
            ))}
          </Box>
        );
      })}

      {/* <TableContainer variant="outlined" component={Paper}>
        <Table sx={{ minWidth: 650 }} aria-label="activity list">
          <TableHead>
            <TableRow>
              <TableCell>Message</TableCell>
              <TableCell>Params</TableCell>
              <TableCell>Result</TableCell>
              <TableCell>Timestamp</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>{items.map(ActivityItemRow)}</TableBody>
        </Table>
      </TableContainer> */}
    </Stack>
  );
};

export default Activity;
