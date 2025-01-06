import * as React from "react";
import Popover from "@mui/material/Popover";
import Typography from "@mui/material/Typography";
import { Box, Button, Paper } from "@mui/material";

interface MouseOverPopoverProps {
  text: string;
  children: React.ReactNode;
}

export default function MouseOverPopover({
  text,
  children,
}: MouseOverPopoverProps) {
  const [anchorEl, setAnchorEl] = React.useState<HTMLElement | null>(null);

  const handlePopoverOpen = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handlePopoverClose = () => {
    setAnchorEl(null);
  };

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(anchorEl ? null : event.currentTarget);
  };

  const open = Boolean(anchorEl);

  return (
    <div>
      <Button
        aria-owns={open ? "mouse-over-popover" : undefined}
        aria-haspopup="true"
        onClick={handleClick}
        // onMouseEnter={handlePopoverOpen}
        // onMouseLeave={handlePopoverClose}
      >
        {text}
      </Button>
      <Popover
        id="mouse-over-popover"
        sx={{
          pointerEvents: "none",
        }}
        open={open}
        anchorEl={anchorEl}
        anchorOrigin={{
          vertical: "bottom",
          horizontal: "left",
        }}
        transformOrigin={{
          vertical: "top",
          horizontal: "left",
        }}
        onClose={handlePopoverClose}
        disableRestoreFocus
      >
        <Paper sx={{ maxHeight: "200px", overflow: "scroll" }}>
          <Typography sx={{ p: 1 }}>{children}</Typography>
        </Paper>
      </Popover>
    </div>
  );
}
