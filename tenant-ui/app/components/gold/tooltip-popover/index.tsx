import { useState } from "react";
import { Button, Popover } from "@mui/material";

export default function CustomizedPopover(props: any) {
  const [anchorEl, setAnchorEl] = useState(null);

  const handleClick = (event: any) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };
  const open = Boolean(anchorEl);

  const id = props.id;
  return (
    <>
      <Button
        aria-describedby={id}
        variant="text"
        onClick={handleClick}
        sx={{ padding: 0, margin: 0, minWidth: 0 }}
      >
        {props.children}
      </Button>
      <Popover
        id={id}
        open={open}
        anchorEl={anchorEl}
        onClose={handleClose}
        anchorOrigin={{
          vertical: "bottom",
          horizontal: "left",
        }}
      >
        {props.content}
      </Popover>
    </>
  );
}
