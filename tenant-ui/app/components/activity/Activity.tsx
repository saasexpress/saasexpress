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
} from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";
import styled from "@emotion/styled";
import TableCell from "components/table-cell";
import CustomizedTooltip from "components/gold/tooltip-simple";
import { useCallback } from "react";
import { useQueryClient } from "@tanstack/react-query";

// const StyledPaper = styled(Paper)(() => ({
//   boxShadow: "none",
// }));

let ActivityItem = (a: any) => {
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

  return (
    <Stack>
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

      <TableContainer variant="outlined" component={Paper}>
        <Table sx={{ minWidth: 650 }} aria-label="activity list">
          <TableHead>
            <TableRow>
              <TableCell>Message</TableCell>
              <TableCell>Params</TableCell>
              <TableCell>Result</TableCell>
              <TableCell>Timestamp</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>{items.map(ActivityItem)}</TableBody>
        </Table>
      </TableContainer>
    </Stack>
  );
};

export default Activity;
