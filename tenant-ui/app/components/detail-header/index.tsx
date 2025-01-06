import { Grid2, Typography, Stack } from "@mui/material";
import { ReactNode } from "react";

interface DetailHeaderProps {
  title: string;
  description: string;
  actions?: ReactNode;
}

const DetailHeader: React.FC<DetailHeaderProps> = ({
  title,
  description,
  actions,
}) => {
  return (
    <Grid2
      container
      direction="row"
      alignItems="center"
      p={0}
      mt={0}
      mb={2}
      sx={{ backgroundColor: "rgb(237, 237, 237)" }}
    >
      <Grid2 p={2} sx={{ mb: 0, backgroundColor: "rgb(237, 237, 237)" }}>
        <Typography variant="h4">{title}</Typography>
        <Typography variant="subtitle1" sx={{ mt: 0.5 }}>
          {description}
        </Typography>
      </Grid2>
      <Grid2 flex={1} textAlign="right" mr={2}>
        <Stack direction="row" spacing={1} justifyContent="flex-end">
          {actions}
        </Stack>
      </Grid2>
    </Grid2>
  );
};

export default DetailHeader;
