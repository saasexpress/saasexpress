import * as React from "react";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogTitle from "@mui/material/DialogTitle";
import BasicButton from "@components/gold/basic-button";
// import { useAppContext } from '@context';
// import APIClient from '@/components/_common/APIClient';

interface WizardDialogProps {
  children: React.ReactNode;
  buttonLabel: string;
  title: React.ReactNode;
  onSubmit: any;
}

export default function YAMLDialog({
  children,
  buttonLabel,
  title,
  onSubmit,
}: WizardDialogProps) {
  //const { session } = useAppContext();

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
        fullWidth
        maxWidth="md"
        open={open}
        onClose={handleClose}
        PaperProps={{
          component: "form",
          onSubmit: (event: React.SyntheticEvent) => {
            event.preventDefault();
            onSubmit();
            handleClose();
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
