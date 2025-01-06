import React from "react";
import PageAlert, { ActionAlert } from "./PageAlert";
import SnackbarAlert from "./Snackbar";

import { Alert, AlertTitle, Box } from "@mui/material";
import GoldLink from "components/gold/link";

const InlineAlert = () => {
  const [state, setState] = React.useState({
    open: false,
    snackbar: false,
    title: "",
    content: [],
    action: undefined,
  } as ActionAlert);

  React.useEffect(() => {
    const subscription = PageAlert.subject.subscribe((e: any) => {
      if (e.action == "alert") {
        setState({ ...state, ...e.alert });
      }
    });

    return () => {
      subscription.unsubscribe();
    };
  }, []);

  function close() {
    PageAlert.doit({ open: false });
  }

  function snackbarClosed() {
    setState({ ...state, ...{ snackbar: false } });
  }

  if (state.open) {
    let action: any = false;
    if ("action" in state && state.action?.link) {
      action = <GoldLink to={state.action.link}>{state.action.label}</GoldLink>;
    }
    return [
      <Alert
        key="page-alert"
        severity="warning"
        action={
          <Box onClick={close}>
            <svg
              //viewPort="0 0 24 24"
              version="1.1"
              width="24"
              height="24"
              style={{ margin: "10px 0" }}
              xmlns="http://www.w3.org/2000/svg"
            >
              <line
                x1="1"
                y1="23"
                x2="23"
                y2="1"
                stroke="black"
                strokeWidth="2"
              />
              <line
                x1="1"
                y1="1"
                x2="23"
                y2="23"
                stroke="black"
                strokeWidth="2"
              />
            </svg>
          </Box>
        }
      >
        <AlertTitle>{state.title}</AlertTitle>
        <Box>
          {state.content?.map((c: string, index: any) => (
            <Box component="span" key={index}>
              {" "}
              | {c}
            </Box>
          ))}
        </Box>
        <Box>{action}</Box>
      </Alert>,
      <SnackbarAlert
        key="snackbar-alert"
        autoHideDuration={2000}
        open={state.snackbar}
        message={state.title}
        content={state.content}
        severity={state.severity}
        onClose={() => snackbarClosed()}
      />,
    ];
  } else {
    return (
      <SnackbarAlert
        key="snackbar-alert-solo"
        open={state.snackbar}
        message={state.title}
        content={state.content}
        severity={state.severity}
        onClose={() => snackbarClosed()}
      />
    );
  }
};

export default InlineAlert;
