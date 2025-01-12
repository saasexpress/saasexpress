import React, { FocusEvent, ChangeEvent, ReactElement } from "react";
import styled from "@emotion/styled";
import { FormControl, TextField, Typography } from "@mui/material";

import WidgetUtils from "../WidgetUtils";

import IkformTypographyText from "@components/typography-text";

const StyledTextField = styled(TextField)``;

class Input extends React.Component<any> {
  constructor(props: any) {
    super(props);
    this.state = { focus: false };
  }

  _onChange(event: any) {
    const key = this.props.value;
    WidgetUtils.setValue(this.props.model, key, event.target.value);

    this.props.onChange(this.props.model);
  }

  // componentWillMount() {
  //   //console.log("INPUT - MOUNT " + this.props.value);
  // }

  // componentWillUnmount() {
  //   //console.log("INPUT - UNMOUNT " + this.props.value);
  // }

  focus(t: boolean) {
    this.setState({ focus: t });
  }

  clear() {
    const { model, value, onChange } = this.props;
    WidgetUtils.setValue(model, value, null);
    onChange(model);
  }

  render() {
    let props = this.props;
    //  ui-autocomplete-loading

    let tooltip = WidgetUtils.tooltip(this.props, () => this.clear());
    const name = this.props.name ? this.props.name : this.props.value;
    let label: ReactElement = <></>;

    if (this.props.label) {
      label = (
        <>
          <span>{this.props.label}</span> {tooltip}
        </>
      );
    }

    let type = this.props.type == "input" ? "text" : this.props.type;

    let key = name;

    let value = WidgetUtils.dot(this.props.model, this.props.value);
    if (value == null) {
      value = "";
    }

    let invalid = this.props.invalid;

    if (this.props.validators) {
      Object.keys(this.props.validators).map((v) => {
        if (v == "REQUIRED") {
          if (value == "") {
            invalid = true;
          } else {
            invalid = false;
          }
        }
      });
    }

    let note: ReactElement = <></>;
    if (this.props.note) {
      note = <div className="note">{this.props.note}</div>;
    }
    //let clazz = this.state.focus ? "input--focus" : "";

    let divClazz = "form-group";
    if (invalid) {
      divClazz = divClazz + " input-invalid";
    }
    // <span className="placeholder-label">{this.props.label}</span>

    if (label) {
      return (
        <FormControl key={key} fullWidth>
          <IkformTypographyText>{label}</IkformTypographyText>
          <StyledTextField
            type={type}
            // ref="input"
            name={name}
            disabled={props?.readonly}
            placeholder={props?.placeholder}
            value={value}
            size="small"
            onBlur={(_: FocusEvent<HTMLInputElement>) => this.focus(false)}
            onFocus={(_: FocusEvent<HTMLInputElement>) => this.focus(true)}
            onChange={(e: ChangeEvent<HTMLInputElement>) => this._onChange(e)}
          />
          {note}
        </FormControl>
      );
    } else {
      if (this.props.readonly == false) {
        return (
          <FormControl key={key} fullWidth>
            <StyledTextField
              type={type}
              // ref="input"
              name={name}
              placeholder={props?.placeholder}
              value={value}
              size="small"
              onBlur={(_: FocusEvent<HTMLInputElement>) => this.focus(false)}
              onFocus={(_: FocusEvent<HTMLInputElement>) => this.focus(true)}
              onChange={(e: ChangeEvent<HTMLInputElement>) => this._onChange(e)}
            />
            <i />
            {note}
          </FormControl>
        );
      } else {
        return (
          <FormControl key={key}>
            <IkformTypographyText>{value}</IkformTypographyText>
            {note}
          </FormControl>
        );
      }
    }
  }
}

export default Input;
