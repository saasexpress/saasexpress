import { Box } from "@mui/material";
import styled from "@emotion/styled";
import { PropsWithChildren } from "react";

const CodePre = styled(Box)(({}) => ({
  backgroundColor: "black",
  color: "white",
  marginTop: "5px",
  whiteSpace: "pre-wrap",
  textAlign: "left",
  overflowWrap: "anywhere",
  fontFamily: "Menlo, Consolas",
  fontSize: "12px",
  // fontWeight: 600,
}));

export default function Code({ children }: PropsWithChildren) {
  return <CodePre p={1}>{children}</CodePre>;
}
