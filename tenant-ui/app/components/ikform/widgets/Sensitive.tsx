import React, {
  Component,
  HTMLElementType,
  useCallback,
  useRef,
  useState,
} from "react";

//import './Sensitive.scss'

import WidgetUtils from "../WidgetUtils";
import IkformTypographyText from "@components/typography-text";
import { Box, FormControl, Stack, TextField } from "@mui/material";
import SquareIcon from "@mui/icons-material/Square";
import VisibilityIcon from "@mui/icons-material/Visibility";
import CopyToClipboardButton from "@components/copy-to-clipboard";
import BasicButton from "@components/gold/basic-button";

const Sensitive = (props: any) => {
  const ref = useRef<HTMLInputElement>(null);

  const [state, setState] = useState({ show: false });

  const _onChange = useCallback((event: any) => {
    const keyValue = props.value;
    WidgetUtils.setValue(props.model, keyValue, event.target.value);

    props.onChange(props.model);
  }, []);

  const toggleShow = useCallback(() => {
    setState({ show: !state.show });
  }, [state]);

  const copy = () => {
    if (ref.current) {
      ref.current.select();
      navigator.clipboard.writeText(ref.current.value);
      ref.current.blur();
    }
  };

  //  ui-autocomplete-loading

  let tooltip = WidgetUtils.tooltip(props);
  // const key = props.key;
  // const name = props.name;
  let label: any = false;
  let value = WidgetUtils.dot(props.model, props.value);

  if (props.label) {
    label = (
      <IkformTypographyText key="sensitive-label">
        {props.label} {tooltip}
      </IkformTypographyText>
    );
  }

  let squares = [...Array(10).keys()].map((k) => (
    <SquareIcon key={k} fontSize="small" />
  ));

  const display = (
    <Stack
      key="sensitive"
      direction="row"
      alignItems="center"
      spacing={0.5}
      sx={{ whiteSpace: "nowrap", color: "#999999" }}
    >
      <BasicButton variant="condensed" onClick={toggleShow}>
        <VisibilityIcon></VisibilityIcon>
      </BasicButton>

      {state.show == false ? (
        <Box flexGrow={1} lineHeight={2.8} paddingTop={0.6}>
          {squares}
        </Box>
      ) : (
        <Box flexGrow={1}>
          <TextField
            fullWidth
            disabled={true}
            value={value}
            onChange={_onChange}
            // sx={{ width: '150px' }}
          />
        </Box>
      )}
      <CopyToClipboardButton value={value} />
      <input ref={ref} type="hidden" value={value} />
    </Stack>
  );
  return (
    <FormControl fullWidth>
      {label}
      {display}
    </FormControl>
  );
};

export default Sensitive;
