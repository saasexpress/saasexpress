import * as React from "react";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogTitle from "@mui/material/DialogTitle";
import BasicButton from "@components/gold/basic-button";
import NewButton from "../new-button";

interface WizardDialogProps {
  children: React.ReactNode;
  buttonLabel: string;
  title: React.ReactNode;
}

export default function WizardDialog({
  children,
  buttonLabel,
  title,
}: WizardDialogProps) {
  const [open, setOpen] = React.useState(false);

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  return (
    <React.Fragment>
      <NewButton onClick={handleClickOpen}>{buttonLabel}</NewButton>
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
        <DialogTitle>{title}</DialogTitle>
        <DialogContent>{children}</DialogContent>
        <DialogActions>
          <BasicButton onClick={handleClose}>Cancel</BasicButton>
          <BasicButton type="submit">Submit</BasicButton>
        </DialogActions>
      </Dialog>
    </React.Fragment>
  );
}
