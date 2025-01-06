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
  Grid,
  Stack,
  Paper,
  Typography,
} from "@mui/material";

import { deepOrange, deepPurple } from "@mui/material/colors";

import { Link } from "react-router";

import BackgroundAvatar from "@components/gold/avatar";

export interface ShellItemProps {
  _id: string;
  label: string;
  status: string;
  description: string;
  building_blocks: string[];
}

export default function ShellPage(item: ShellItemProps) {
  let blocks =
    "building_blocks" in item
      ? item.building_blocks.map((t) => (
          <Grid item>
            <Chip
              key={t}
              label={t}
              variant="filled"
              avatar={
                <Avatar sx={{ color: "white", bgcolor: deepPurple[300] }}>
                  {t
                    .split(" ")
                    .map((m) => m.substring(0, 1))
                    .join()
                    .toUpperCase()}
                </Avatar>
              }
              sx={{ minWidth: "60px" }}
            />
          </Grid>
        ))
      : false;

  return (
    <Grid item xs={12} sm={6} lg={3} md={3} key={item._id}>
      <Link to={{ pathname: `/shell/${item._id}/settings` }}>
        <Card
          raised={false}
          sx={{
            minHeight: 220,
            // border: 'thin solid',
            // borderImageSource:
            //   'linear-gradient(180deg,#ff38bb 5%,#ff8038 55%,#f5be66 65%,#f1fff7 93%)',
            // borderImageSlice: 1,
            // borderImageWidth: 1,
          }}
        >
          <CardContent sx={{ padding: 1 }}>
            <Box>
              {/* <div className="pull-right">
                  <Metric
                    key="blocks"
                    label="BLOCKS"
                    count={item.building_blocks.length}
                  />
                </div> */}
              <Stack direction="row" spacing={1}>
                <BackgroundAvatar>{item.label}</BackgroundAvatar>
                <Typography
                  sx={{
                    fontSize: "180%",
                    textOverflow: "ellipsis",
                    whiteSpace: "nowrap",
                  }}
                >
                  {item.label}
                </Typography>
              </Stack>{" "}
              <div className="es-status">{item.status}</div>
              {/* {false ? (
                <span>
                  <Rating rating={{ positive: 10, negative: 100 }} />
                </span>
              ) : (
                false
              )}
              {false ? (
                <Stack
                  direction="row"
                  justifyContent="center"
                  spacing={1}
                  mt={2}
                >
                  {tags}
                </Stack>
              ) : (
                false
              )} */}
              {/* <Stack direction="row" spacing={1} alignItems="center">
                {versions}
              </Stack> */}
              <Grid
                container
                direction="row"
                justifyContent="center"
                spacing={1}
                padding={0}
                margin={0}
                mt={2}
              >
                {blocks}
              </Grid>
            </Box>
          </CardContent>
        </Card>
      </Link>
    </Grid>
  );
}
