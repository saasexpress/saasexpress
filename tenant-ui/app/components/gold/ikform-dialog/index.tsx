import * as React from "react";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogTitle from "@mui/material/DialogTitle";
import BasicButton from "@components/gold/basic-button";
import APIClient from "@lib/api/APIClient";
//import { useAppContext } from '@context.tsx';
import FormRenderer from "@components/ikform/FormRenderer";
import AllExtensions from "components/ikform/Extensions";
//import CCIWidgets from '../../forms/bundles/cci.js';
//import AllWidgets from '@/ikform/forms/bundles/all';

interface IkformDialogProps {
  title: string;
  layout: string;
  data?: any;
  onSubmit?: any;

  requestOpen?: boolean;
  onClose: any;
}

export default function IkformDialog({
  title,
  layout,
  data = {},
  onSubmit,
  requestOpen = false,
  onClose,
}: IkformDialogProps) {
  const ref = React.useRef(undefined);
  //const { session } = useAppContext();
  const [open, setOpen] = React.useState(false);

  const [form, setForm] = React.useState({ ui: undefined });

  const [model, setModel] = React.useState({ ...data });

  const handleClose = () => {
    setOpen(false);
    onClose();
  };

  React.useEffect(() => {
    setOpen(requestOpen);
  }, [requestOpen]);

  // React.useEffect(() => {
  //   setModel({ ...data });
  // }, [data]);

  React.useEffect(() => {
    const results = APIClient().get(
      ["ikform-layout", layout],
      "/api/ikform/layouts/" + layout + "?rand=" + Math.random()
    );

    setForm(results.data?.data);
    setModel(data);
  }, [layout, requestOpen]);

  const modelOnChange = React.useCallback((upd: any) => {
    setModel({ ...model, ...upd });
  }, []);

  //const extensions = AllWidgets.concat(CCIWidgets as any[]);

  return (
    <Dialog
      open={open}
      onClose={handleClose}
      PaperProps={{
        component: "form",
        onSubmit: (event: React.SyntheticEvent) => {
          event.preventDefault();
          const formData = new FormData(event.currentTarget as HTMLFormElement);
          const formJson = Object.fromEntries(formData.entries());
          console.log(JSON.stringify(formJson));
          setModel(undefined);
          handleClose();
          if (onSubmit) {
            onSubmit(formJson);
          }
        },
      }}
    >
      <DialogTitle>{title}</DialogTitle>
      <DialogContent>
        {model && form.ui && (
          <FormRenderer
            ref={ref}
            model={model}
            ui={form.ui}
            onAction={(ev: any, action: any, c: any) => {
              console.log(
                "FormEvent " +
                  action +
                  " detail keys: [ " +
                  Object.keys(c || {}) +
                  " ]"
              );
            }}
            extensions={AllExtensions}
            onChange={(e: any) => modelOnChange(e)}
          />
        )}
      </DialogContent>
      <DialogActions>
        <BasicButton variant="text" onClick={handleClose}>
          Cancel
        </BasicButton>
        <BasicButton type="submit">Submit</BasicButton>
      </DialogActions>
    </Dialog>
  );
}
