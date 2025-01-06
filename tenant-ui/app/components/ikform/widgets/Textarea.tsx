import React from "react";
import { TextField } from "@mui/material";

import WidgetUtils from "../WidgetUtils";
import IkformTypographyText from "@components/typography-text";

class Textarea extends React.Component<any> {
  constructor(props: any) {
    super(props);
    this.state = { focus: false };
  }

  _onChange(event: any) {
    const key = this.props.value;
    WidgetUtils.setValue(this.props.model, key, event.target.value);
    this.props.onChange(this.props.model);
  }

  focus(t: any) {
    this.setState({ focus: t });
  }

  render() {
    let tooltip = WidgetUtils.tooltip(this.props);
    const name = this.props.name;
    let label = <></>;
    let value = WidgetUtils.dot(this.props.model, this.props.value);
    if (value == null) {
      value = "";
    }

    //const style = { minHeight: "102px" };

    if (this.props.label) {
      label = (
        <IkformTypographyText>
          {this.props.label} {tooltip}
        </IkformTypographyText>
      );
    }

    let clazz = (this.state as any).focus
      ? "custom-scroll textarea--focus"
      : "custom-scroll";

    if (this.props.cssClass) {
      clazz += " " + this.props.cssClass;
    }

    let divClazz = "form-group";
    if (this.props.invalid) {
      divClazz = divClazz + " input-invalid";
    }

    return (
      <section key={name} className={divClazz}>
        {label}
        <TextField
          fullWidth
          multiline
          inputProps={{ style: { resize: "both" } }}
          minRows={4}
          name={name}
          value={value}
          sx={{ padding: 0 }}
          onBlur={() => this.focus(false)}
          onFocus={() => this.focus(true)}
          onChange={(e: Event) => this._onChange(e)}
        ></TextField>
      </section>
    );
  }
}

export default Textarea;
