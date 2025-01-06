import { Link, useNavigate } from "react-router";
import { Grid2, Stack, Typography } from "@mui/material";
import LayersIcon from "@mui/icons-material/Layers";
import { useQueryClient } from "@tanstack/react-query";
import Actions from "./actions";
import useAPIClient from "lib/api/APIClient";
import APIErrorHandler from "lib/alerts/APIErrorHandler";
//import APIErrorHandler from "lib/alerts/APIErrorHandler";

interface HeaderProps {
  name?: string;
  id: string;
}

export default function Header({ name, id }: HeaderProps) {
  const api = useAPIClient();
  const queryClient = useQueryClient();
  const navigate = useNavigate();

  const handleDelete = async () => {
    api.delete(`/api/tenants/${id}`, () => {
      queryClient.cancelQueries({ queryKey: ["tenant", id] });
      queryClient.invalidateQueries({ queryKey: ["tenants"] });
      // does not work
      APIErrorHandler.notice({
        title: "Tenant",
        content: "Deleted successfully",
      });
      navigate("/tenants");
    });
  };
  return (
    <Grid2
      container
      direction="row"
      alignItems="top"
      p={4}
      mt={4}
      sx={{ backgroundColor: "rgb(237, 237, 237)" }}
    >
      <Grid2>
        <Stack direction="row" alignItems="center" justifyContent="left" mb={1}>
          <LayersIcon fontSize="large" />

          <Typography variant="h3" paddingLeft={0}>
            {name ? `Tenant ${name}` : "New Tenant"}
          </Typography>
        </Stack>
        <Link to={{ pathname: "/tenants" }}>
          <Typography variant="link">List of Tenants</Typography>
        </Link>
      </Grid2>
      <Grid2 flex={1} textAlign="right">
        <Actions handleDelete={handleDelete} />
      </Grid2>
    </Grid2>
  );
}
