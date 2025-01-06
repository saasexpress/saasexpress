import * as React from "react";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogTitle from "@mui/material/DialogTitle";
import BasicButton from "@components/gold/basic-button";

interface SaveFormDialogProps {
  children: React.ReactNode;
  buttonLabel: string;
  title: string;
  onSubmit?: any;
}

export default function SaveFormDialog({
  children,
  buttonLabel,
  title,
  onSubmit,
}: SaveFormDialogProps) {
  const [open, setOpen] = React.useState(false);

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  return (
    <React.Fragment>
      <BasicButton onClick={handleClickOpen}>{buttonLabel}</BasicButton>
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
            if (onSubmit) {
              onSubmit(formJson);
            }
          },
        }}
      >
        <DialogTitle>{title}</DialogTitle>
        <DialogContent>{children}</DialogContent>
        <DialogActions>
          <BasicButton variant="text" onClick={handleClose}>
            Cancel
          </BasicButton>
          <BasicButton type="submit">Submit</BasicButton>
        </DialogActions>
      </Dialog>
    </React.Fragment>
  );
}
