import styled from "@emotion/styled";
import { Theme } from "@mui/material/styles";
import TableCell, { tableCellClasses } from "@mui/material/TableCell";

const IkformTableCell = styled(TableCell)(({ theme }: { theme?: Theme }) => ({
  // [`&.${tableCellClasses.head}`]: {
  //   fontSize: "0.9rem",
  //   color: theme?.palette.primary.main,
  //   fontWeight: 600,
  // },
  // [`&.${tableCellClasses.body}`]: {
  //   fontSize: "1rem",
  // },
}));

export default IkformTableCell;
