import { Link, useNavigate } from "react-router";
import { Grid2, Stack, Typography } from "@mui/material";
import LayersIcon from "@mui/icons-material/Layers";
import { useQueryClient } from "@tanstack/react-query";
import Actions from "./actions";
import useAPIClient from "lib/api/APIClient";
import APIErrorHandler from "lib/alerts/APIErrorHandler";
import InPlaceEdit from "components/inplace-edit";
import { useCallback } from "react";
//import APIErrorHandler from "lib/alerts/APIErrorHandler";

interface HeaderProps {
  name?: string;
  id: string;
}

export default function Header({ name, id }: HeaderProps) {
  const api = useAPIClient();
  const queryClient = useQueryClient();
  const navigate = useNavigate();

  const handleSaveName = useCallback(
    async (displayName: string) => {
      api.put(`/api/services/${id}`, { displayName }, () => {
        queryClient.invalidateQueries({ queryKey: ["service", id] });
        queryClient.invalidateQueries({ queryKey: ["services"] });
        queryClient.removeQueries({ queryKey: ["list-activity"] });
        APIErrorHandler.notice({
          title: "Service",
          content: "Updated successfully",
        });
      });
    },
    [name, id]
  );

  const handleDelete = async () => {
    api.delete(`/api/services/${id}`, () => {
      queryClient.cancelQueries({ queryKey: ["service", id] });
      queryClient.invalidateQueries({ queryKey: ["services"] });
      queryClient.removeQueries({ queryKey: ["list-activity"] });
      APIErrorHandler.notice({
        title: "Service",
        content: "Deleted successfully",
      });
      navigate("/services");
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

          <Typography variant="h3" paddingLeft={1}>
            <InPlaceEdit key={id} initialValue={name} onSave={handleSaveName} />
          </Typography>
        </Stack>
        <Link to={{ pathname: "/services" }}>
          <Typography variant="link">List of Services</Typography>
        </Link>
      </Grid2>
      <Grid2 flex={1} textAlign="right">
        <Actions handleDelete={handleDelete} />
      </Grid2>
    </Grid2>
  );
}
