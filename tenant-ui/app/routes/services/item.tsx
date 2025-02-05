import React from "react";

import {
  Avatar,
  Box,
  Chip,
  Card,
  CardContent,
  Grid2,
  Stack,
  Paper,
  Typography,
} from "@mui/material";

import { Link } from "react-router";

import BackgroundAvatar from "@components/gold/avatar";

export interface ServiceItemProps {
  id: string;
  displayName: string;
}

export default function Item(item: ServiceItemProps) {
  return (
    <Grid2 size={{ xs: 12, sm: 6, lg: 12, md: 12 }} key={item.id}>
      <Link to={{ pathname: `/services/${item.id}/editor` }}>
        <Card
          raised={false}
          sx={{
            minHeight: 160,
          }}
        >
          <CardContent sx={{ padding: 1 }}>
            <Stack direction="column" spacing={1}>
              <Stack direction="row" spacing={1}>
                <BackgroundAvatar>{item.displayName}</BackgroundAvatar>
                <Typography
                  title={item.displayName}
                  sx={{
                    fontSize: "180%",
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    whiteSpace: "initial",
                    WebkitLineClamp: "1",
                    WebkitBoxOrient: "vertical",
                  }}
                >
                  {item.displayName}
                </Typography>
              </Stack>
              <Typography fontSize={10}>{item.id}</Typography>
            </Stack>
          </CardContent>
        </Card>
      </Link>
    </Grid2>
  );
}
