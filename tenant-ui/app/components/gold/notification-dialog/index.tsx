import * as React from "react";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogTitle from "@mui/material/DialogTitle";
import BasicButton from "@components/gold/basic-button";
import NotificationsIcon from "@mui/icons-material/Notifications";
import { Box, Stack, styled } from "@mui/material";
import DetailHeader from "@components/detail-header";

const NotifyIcon = styled(Stack)(({ theme }) => ({
  // padding: theme.spacing(12),
  padding: "5px",
  textAlign: "center",
  color: "rgb(0, 95, 115)",
  cursor: "pointer",
  "&:hover, &.Mui-focusVisible": {
    backgroundColor: "rgb(0, 95, 115)",
    color: "white",
    borderRadius: "3px",
    //boxShadow: `0px 0px 0px 8px ${alpha(theme.palette.success.main, 0.16)}`,
  },
  // color: theme.palette.text.primary,
}));

interface NotificationDialogProps {
  children: React.ReactNode;
  title: string;
  description: string;
}

export default function NotificationDialog({
  children,
  title,
  description,
}: NotificationDialogProps) {
  const [open, setOpen] = React.useState(false);

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  return (
    <React.Fragment>
      <NotifyIcon
        onClick={handleClickOpen}
        spacing={4}
        direction="row"
        justifyContent="center"
        alignItems="center"
        fontSize="large"
      >
        <NotificationsIcon fontSize="large"></NotificationsIcon>
      </NotifyIcon>
      <Dialog
        open={open}
        onClose={handleClose}
        PaperProps={{
          component: "form",
          onSubmit: (event: React.SyntheticEvent) => {
            event.preventDefault();
            const formData = new FormData(
              event.currentTarget as HTMLFormElement
            );
            const formJson = Object.fromEntries(formData.entries());
            console.log(JSON.stringify(formJson));
            handleClose();
          },
        }}
      >
        <DialogTitle>
          <DetailHeader title={title} description={description} />
        </DialogTitle>
        <DialogContent>{children}</DialogContent>
        <DialogActions>
          <BasicButton onClick={handleClose}>Close</BasicButton>
        </DialogActions>
      </Dialog>
    </React.Fragment>
  );
}
