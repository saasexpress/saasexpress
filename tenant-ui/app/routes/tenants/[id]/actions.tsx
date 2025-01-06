import {
  alpha,
  Box,
  Button,
  Menu,
  MenuItem,
  Stack,
  styled,
} from "@mui/material";
import Link from "components/gold/link";
import DeleteForeverIcon from "@mui/icons-material/DeleteForever";
import SaveFormDialog from "components/gold/save-dialog";
import NotificationDialog from "components/gold/notification-dialog";
import MoreVertIcon from "@mui/icons-material/MoreVert";
import { useState } from "react";

const NotifyIcon = styled(Stack)(({ theme }) => ({
  padding: theme.spacing(1),
  textAlign: "center",
  color: theme.palette.text.primary,
  //color: "rgb(0, 95, 115)",
  cursor: "pointer",
  "&:hover, &.Mui-focusVisible": {
    backgroundColor: "rgb(0, 95, 115)",
    color: "white",
    borderRadius: "3px",
    boxShadow: `0px 0px 0px 8px ${alpha(theme.palette.success.main, 0.16)}`,
  },
}));

export default function Actions({ handleDelete }: any) {
  const [anchorEl, setAnchorEl] = useState(null);
  const open = Boolean(anchorEl);

  const handleClick = (event: any) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };

  const tabKeys = {
    blocks: "Building Blocks",
    variants: "Variants",
    activity: "Activity",
    settings: "Settings",
  };
  return (
    <Stack direction="row" spacing={1} justifyContent="flex-end">
      {/* <NotifyIcon>
        <DeleteForeverIcon fontSize="large" sx={{}} />
      </NotifyIcon> */}
      {/* <Link to="/tenants">
      </Link> */}
      <SaveFormDialog
        onSubmit={handleDelete}
        buttonLabel="Delete"
        title="Confirm delete Tenant"
      >
        <> </>
      </SaveFormDialog>
      {/* <NotificationDialog
        title={"Confirm delete Tenant"}
        description={`Delete?`}
      >
        <Box>blah</Box>
      </NotificationDialog> */}
      <Stack direction="column" spacing={0} alignItems="flex-end">
        <Button
          id="basic-button"
          aria-controls={open ? "basic-menu" : undefined}
          aria-haspopup="true"
          aria-expanded={open ? "true" : undefined}
          onClick={handleClick}
        >
          <MoreVertIcon />
        </Button>
        ,
        <Menu
          id="basic-menu"
          anchorEl={anchorEl}
          open={open}
          onClose={handleClose}
          MenuListProps={{
            "aria-labelledby": "basic-button",
          }}
        >
          {Object.entries(tabKeys).map(([key, label], index) => (
            <Link key={index} to={`/${key}`}>
              <MenuItem>{label}</MenuItem>
            </Link>
          ))}
        </Menu>
      </Stack>
    </Stack>
  );
}
