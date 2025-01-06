import * as React from "react";
import { FormControl, Switch, Theme } from "@mui/material";
import styled from "@emotion/styled";

import WidgetUtils from "../WidgetUtils";
import IkformTypographyText from "@components/typography-text.tsx";

const AntSwitch = styled(Switch)(({ theme }: any) => ({
  root: {
    width: 44,
    height: 22,
    padding: 0,
    margin: 0,
    display: "flex",
  },
  switchBase: {
    padding: 2,
    color: theme.palette.grey[500],
    "&$checked": {
      transform: "translateX(22px)",
      color: theme.palette.common.white,
      "& + $track": {
        opacity: 1,
        backgroundColor: theme.palette.primary.main,
        borderColor: theme.palette.primary.main,
      },
    },
  },
  thumb: {
    width: 16,
    height: 16,
    margin: 1,
    boxShadow: "none",
  },
  track: {
    border: `1px solid ${theme.palette.grey[500]}`,
    borderRadius: 44 / 2,
    opacity: 1,
    backgroundColor: theme.palette.common.white,
  },
  checked: {},
}));

class Toggle extends React.Component<any> {
  _onChange(event: any) {
    const key = this.props.value;
    const dom = event.target; //ReactDOM.findDOMNode(this.refs.checkbox);

    WidgetUtils.setValue(this.props.model, key, dom.checked ? true : false);

    this.props.onChange(this.props.model);
  }

  reset() {}

  clear() {
    WidgetUtils.setValue(this.props.model, this.props.value, null);
    this.props.onChange(this.props.model);
  }

  render() {
    //  ui-autocomplete-loading

    let tooltip = false; // WidgetUtils.tooltip(this.props, (e) => this.clear());
    const name = this.props.name;
    let label = <></>;
    let value = WidgetUtils.dot(this.props.model, this.props.value);

    let note = <></>;
    if (this.props.note) {
      note = <div className="note">{this.props.note}</div>;
    }

    if (this.props.label) {
      label = (
        <IkformTypographyText>
          {this.props.label} {tooltip}
        </IkformTypographyText>
      );
    }

    let clazz = "toggle ikform-toggle";

    let disabled: boolean | undefined = false;
    if (this.props.readonly) {
      clazz = clazz + " state-disabled";
      disabled = true;
    }

    let divClazz = "ikform-group";
    if (this.props.invalid) {
      divClazz = divClazz + " input-invalid";
    }

    if (label) {
      return (
        <FormControl sx={{ gap: 1 }} key={name}>
          {label}
          <Switch
            disableRipple
            name={name}
            defaultValue={"true"}
            checked={value == true}
            onChange={(e) => this._onChange(e)}
          />
        </FormControl>
      );
    } else {
      return (
        <FormControl sx={{ gap: 0.5 }} key={name}>
          <Switch
            disabled={disabled}
            disableRipple
            name={name}
            defaultValue={"true"}
            checked={value == true}
            onChange={(e) => this._onChange(e)}
          />
        </FormControl>
      );
    }
  }
}

export default Toggle;
