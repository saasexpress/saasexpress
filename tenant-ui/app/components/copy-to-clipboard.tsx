import { Alert, Snackbar } from "@mui/material";
import { useState } from "react";
import CopyIcon from "@mui/icons-material/ContentCopy";
import BasicButton from "@components/gold/basic-button";

interface CopyToClipboardButtonProps {
  value: string;
}

const CopyToClipboardButton = ({ value }: CopyToClipboardButtonProps) => {
  const [open, setOpen] = useState(false);
  const handleClick = () => {
    setOpen(true);
    navigator.clipboard.writeText(value);
  };

  return (
    <>
      <BasicButton variant="condensed" onClick={handleClick}>
        <CopyIcon></CopyIcon>
      </BasicButton>
      <Snackbar
        anchorOrigin={{ vertical: "bottom", horizontal: "left" }}
        open={open}
        onClose={() => setOpen(false)}
        autoHideDuration={2000}
      >
        <Alert severity="success" variant="filled" sx={{ width: "100%" }}>
          Copied to clipboard
        </Alert>
      </Snackbar>
    </>
  );
};

export default CopyToClipboardButton;
