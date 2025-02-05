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
import { Link } from "react-router";
import RefreshIcon from "@mui/icons-material/Refresh";
import TableCell from "components/table-cell";
import { useCallback } from "react";
import { useQueryClient } from "@tanstack/react-query";

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

let ServiceItemRow = (s: any) => {
  // const { changes, version, count } = renderChanges(s.service);

  let ts = <TimeAgo date={new Date(s.serviceAt)} />;

  return (
    <TableRow
      key={s.id}
      sx={{
        "&:last-child td, &:last-child th": { border: 0 },
        verticalAlign: "top",
      }}
    >
      <TableCell>
        <Link to={{ pathname: `/services/${s.id}/editor` }}>{s.id}</Link>
      </TableCell>
      <TableCell>{s.displayName}</TableCell>
    </TableRow>
  );
};

let Service = ({ data, handleChangePage, handleChangeRowsPerPage }: any) => {
  const { items, paging } = data;

  const query = useQueryClient();

  const doRefresh = useCallback(
    (_: any) => {
      query.invalidateQueries({ queryKey: ["list-services"] });
    },
    [paging]
  );

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

      <TableContainer variant="outlined" component={Paper}>
        <Table sx={{ minWidth: 650 }} aria-label="service list">
          <TableHead>
            <TableRow>
              <TableCell>ID</TableCell>
              <TableCell>Name</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>{items.map(ServiceItemRow)}</TableBody>
        </Table>
      </TableContainer>
    </Stack>
  );
};

export default Service;
