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
import React from "react";


import ChevronDownIcon from "@mui/icons-material/ExpandMore";
import ChevronUpIcon from "@mui/icons-material/ExpandLess";
import Code from "components/gold/code";

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
  const [openId, setOpenId] = React.useState<string | null>();
  // const { changes, version, count } = renderChanges(s.service);

  const handleDetailsDisclosure = React.useCallback(
    (id: string) => () => {
      setOpenId((state) => (state !== id ? id : null));
    },
    [setOpenId]
  );

  let ts = <TimeAgo date={new Date(s.serviceAt)} />;

  return [
    <TableRow
      key={s.id}
      sx={{
        "&:last-child td, &:last-child th": { border: 0 },
        verticalAlign: "top",
      }}
    >
      <TableCell>
        <Link to={{ pathname: `/services/${s.id}/editor` }}>
          {s.displayName}
        </Link>
        </TableCell>
      <TableCell sx={{ textAlign: "right" }}>
        <IconButton
          onClick={handleDetailsDisclosure(s.id)}
        >
          {s.id == openId ? <ChevronUpIcon /> : <ChevronDownIcon />}
        </IconButton>
      </TableCell>
    </TableRow>,
    s.id == openId && (
      <TableRow>
        <TableCell colSpan={2}>
          <Stack direction="row" spacing={2} alignItems="center">
            <Stack>
              <div>Service ID: {s.id}</div>
              <div>Service URL: {s.serviceUrl}</div>
              <div>Variants:</div>
              {Object.values(s.variants).map((v: any) => (
                  <Link key={v.dag.name} to={{ pathname: `/services/${s.id}/editor`, search: `variant=${v.dag.name}` }}>
                    {v.dag.name}
                  </Link>
              ))}
            </Stack>
          </Stack>
        </TableCell>
      </TableRow>
    )
    
  ];
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
        <Table  aria-label="service list">
          <TableHead>
            <TableRow>
              <TableCell>Name</TableCell>
              <TableCell></TableCell>
            </TableRow>
          </TableHead>
          <TableBody>{items.map(ServiceItemRow)}</TableBody>
        </Table>
      </TableContainer>
    </Stack>
  );
};

export default Service;
