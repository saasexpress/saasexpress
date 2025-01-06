import React from "react";
import {
  Alert as MuiAlert,
  Snackbar,
  Typography,
  Box,
  SnackbarCloseReason,
} from "@mui/material";
import Slide, { SlideProps } from "@mui/material/Slide";
// import CloseIcon from "@mui/icons-material/Close";
// import { useAppContext } from "context";
// import { makeStyles } from "@mui/styles"

// import Slide from "@mui/material/Slide"

// const log = _log('snackbar');
// const log = {
//   debug: (str) => {
//     console.log(str);
//   },
// };

// function TransitionUp(props) {
//   return <Slide {...props} direction="up" />
// }

// const useStyles = makeStyles((theme) => ({
//   root: {
//     width: "100%",
//     "& > * + *": {
//       marginTop: "2",
//     },
//   },
// }))

export default function CustomizedSnackbars(props: any) {
  //const { log: _log } = useAppContext();
  //const log = _log("snackbar");

  // const classes = useStyles()
  const [open, setOpen] = React.useState(props.open);
  //  const [transition, setTransition] = React.useState(undefined);

  React.useEffect(() => {
    //log.debug("%s %s", "Effect", JSON.stringify(props));
    setOpen(props.open);
  }, [props]);

  // const handleClick = (Transition: any) => () => {
  //   setTransition(() => Transition);
  //   setOpen(true);
  // };

  const handleClose = (
    _: React.SyntheticEvent | Event,
    reason?: SnackbarCloseReason
  ) => {
    if (reason === "clickaway") {
      return;
    }

    setOpen(false);
    props.onClose();
  };

  // const action = (
  //   <IconButton
  //     size="small"
  //     aria-label="close"
  //     color="inherit"
  //     onClick={handleClose}
  //   >
  //     <CloseIcon fontSize="small" />
  //   </IconButton>
  // );
  return (
    <Snackbar
      anchorOrigin={{
        vertical: "bottom",
        horizontal: "center",
      }}
      open={open}
      TransitionComponent={Slide}
      autoHideDuration={2000}
      onClose={handleClose}
    >
      <MuiAlert
        // onClose={handleClose}
        elevation={6}
        variant="filled"
        severity={props.severity}
        sx={{ width: "100%" }}
      >
        <Box>
          <Typography>{props.message}</Typography>
          {props.content ? (
            <Typography>
              {props.content.map((c: any, index: any) => (
                <Box component="span" key={index}>
                  {" "}
                  | {c}
                </Box>
              ))}
            </Typography>
          ) : (
            false
          )}
        </Box>
      </MuiAlert>
    </Snackbar>
  );
}
