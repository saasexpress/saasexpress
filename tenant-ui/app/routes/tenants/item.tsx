import React from "react";

import {
  IntlProvider,
  FormattedDate,
  FormattedTime,
  FormattedNumber,
  FormattedPlural,
} from "react-intl";

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

import { deepOrange, deepPurple } from "@mui/material/colors";

import { Link } from "react-router";

import BackgroundAvatar from "@components/gold/avatar";

export interface TenantItemProps {
  id: string;
  displayName: string;
}

export default function Item(item: TenantItemProps) {
  return (
    <Grid2 size={{ xs: 12, sm: 6, lg: 3, md: 3 }} key={item.id}>
      <Link to={{ pathname: `/tenants/${item.id}/profile` }}>
        <Card
          raised={false}
          sx={{
            minHeight: 160,
            // border: "thin solid",
            // borderImageSource:
            //   "linear-gradient(180deg,#ff38bb 5%,#ff8038 55%,#f5be66 65%,#f1fff7 93%)",
            // borderImageSlice: 1,
            // borderImageWidth: 1,
          }}
        >
          <CardContent sx={{ padding: 1 }}>
            <Stack direction="column" spacing={1}>
              <Stack direction="row" spacing={1}>
                <BackgroundAvatar>{item.displayName}</BackgroundAvatar>
                <Typography
                  sx={{
                    fontSize: "180%",
                    textOverflow: "ellipsis",
                    whiteSpace: "nowrap",
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
