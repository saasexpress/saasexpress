import * as React from "react";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import NewButton from "@components/gold/new-button";
import BasicButton from "@components/gold/basic-button";
import APIClient from "@lib/api/APIClient";
import SimpleTextInput from "components/simple-text-input";
import { FormControl, FormGroup, FormLabel } from "@mui/material";

export default function NewServiceDialog({ onCreated }: any) {
  const [open, setOpen] = React.useState(false);

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  const onSubmit = React.useCallback((event: any) => {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const formJson = Object.fromEntries(formData.entries());
    APIClient().post("/api/services", formJson, (d: any) => {
      onCreated(d);
    });
    handleClose();
  }, []);

  return (
    <React.Fragment>
      <NewButton onClick={handleClickOpen}>New service</NewButton>
      <form>
        <Dialog
          open={open}
          onClose={handleClose}
          PaperProps={{
            component: "form",
            onSubmit: onSubmit,
          }}
        >
          <DialogTitle>Create New Service</DialogTitle>
          <DialogContent>
            <DialogContentText mb="2" className="mt-2">
              To create a new service, enter an optional display name and
              service URL:
            </DialogContentText>
            <FormGroup>
              <FormControl>
                <FormLabel>Display Name</FormLabel>
                <SimpleTextInput name="displayName" onChange={() => true} />
              </FormControl>

              <FormControl required>
                <FormLabel>Service URL</FormLabel>
                <SimpleTextInput name="serviceUrl" onChange={() => true} />
              </FormControl>
            </FormGroup>
          </DialogContent>
          <DialogActions>
            <BasicButton color="secondary" variant="text" onClick={handleClose}>
              Cancel
            </BasicButton>
            <BasicButton type="submit">Submit</BasicButton>
          </DialogActions>
        </Dialog>
      </form>
    </React.Fragment>
  );
}
