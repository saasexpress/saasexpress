import { Link } from "react-router";
import { LinkOwnProps, Link as MuiLink } from "@mui/material";
import { ReactNode } from "react";

interface GoldLinkProps extends LinkOwnProps {
  to: string;
  children: ReactNode;
}

const GoldLink = ({ to, children, underline = "none" }: GoldLinkProps) => (
  // <MuiLink underline={underline}>
  <Link to={{ pathname: to }}>{children}</Link>
  // </MuiLink>
);

export default GoldLink;
