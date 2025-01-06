import * as React from "react";
import Avatar from "@mui/material/Avatar";
import Stack from "@mui/material/Stack";

function stringToColor(string: string) {
  let hash = 0;
  let i;

  /* eslint-disable no-bitwise */
  for (i = 0; i < string.length; i += 1) {
    hash = string.charCodeAt(i) + ((hash << 5) - hash);
  }

  let color = "#";

  for (i = 0; i < 3; i += 1) {
    const value = (hash >> (i * 8)) & 0xff;
    color += `00${value.toString(16)}`.substr(-2);
  }
  /* eslint-enable no-bitwise */

  return color;
}

function stringAvatar(name?: string) {
  const parts = name ? name.toUpperCase().split(" ") : ["Undefined"];
  return {
    sx: {
      //bgcolor: stringToColor(name),
    },
    children: `${parts[0][0]}${parts.length > 1 ? parts[1][0] : ""}`,
  };
}

export default function BackgroundLetterAvatars({ children }: any) {
  return <Avatar {...stringAvatar(children)} />;
}
