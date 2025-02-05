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

export default function NewTenanttDialog({ onCreated }: any) {
  // const { session } = useAppContext();
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
    APIClient().post("/api/tenants", formJson, (d: any) => {
      onCreated(d);
    });
    handleClose();
  }, []);

  return (
    <React.Fragment>
      <NewButton onClick={handleClickOpen}>New tenant</NewButton>
      <form>
        <Dialog
          open={open}
          onClose={handleClose}
          PaperProps={{
            component: "form",
            onSubmit: onSubmit,
          }}
        >
          <DialogTitle>Create New Tenant</DialogTitle>
          <DialogContent>
            <DialogContentText>
              To create a new tenant, enter an optional display name:
            </DialogContentText>

            <SimpleTextInput name="displayName" onChange={() => true} />
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
