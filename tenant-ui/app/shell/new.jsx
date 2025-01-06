import * as React from 'react';
import TextField from '@mui/material/TextField';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogTitle from '@mui/material/DialogTitle';
import NewButton from '@/components/gold/new-button';
import BasicButton from '@/components/gold/basic-button';
import SimpleTextInput from '@/components/ikform/cards/shared/input';
import APIClient from '@/components/_common/APIClient';
import { useAppContext } from '@/lib/context';
import APIErrorHandler from '@/components/_common/APIErrorHandler';

export default function NewBlueprintDialog({ onCreated }) {
  const { session } = useAppContext();
  const [open, setOpen] = React.useState(false);

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  const onSubmit = React.useCallback((event) => {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const formJson = Object.fromEntries(formData.entries());
    APIClient(session).handleSubmit('/api/v2/blueprints', formJson, (d) => {
      APIErrorHandler.notice({
        title: 'Blueprint',
        content: 'Added successfully.',
      });
    });
    handleClose();
    onCreated();
  }, []);

  return (
    <React.Fragment>
      <NewButton onClick={handleClickOpen}>New blueprint</NewButton>
      <form>
        <Dialog
          open={open}
          onClose={handleClose}
          PaperProps={{
            component: 'form',
            onSubmit: onSubmit,
          }}
        >
          <DialogTitle>Create New Blueprint</DialogTitle>
          <DialogContent>
            <DialogContentText>
              To create a new blueprint, enter a name below:
            </DialogContentText>

            <SimpleTextInput
              name="label"
              label="Label"
              type="text"
              fullWidth
              onChange={() => true}
            />
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
