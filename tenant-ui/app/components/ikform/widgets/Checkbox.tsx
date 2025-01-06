import React, { createRef, ReactElement } from "react";
import WidgetUtils from "../WidgetUtils";

import { observer } from "mobx-react";
import { listsState } from "../ListStore";

import Checkbox from "@mui/material/Checkbox";
import TheCheckboxIcon from "@mui/icons-material/CheckBox";
import TheCheckBoxOutlineBlank from "@mui/icons-material/CheckBoxOutlineBlank";

//import { createSvgIcon } from "@mui/material/utils";
import IkformTypographyText from "@components/typography-text";
import { FormControl, FormControlLabel } from "@mui/material";
//import IkformSpanText from "@components/span-text";

// const CheckBoxIcon = createSvgIcon(
//   <path
//     transform="scale(0.7) translate(7,4)"
//     strokeWidth="1"
//     stroke="#00688b"
//     d="M24 24h-24v-24h24v24zm-1-23h-22v22h22v-22zm-3 6.435l-10.005 10.565-4.995-5.866.761-.648 4.271 5.015 9.24-9.751.728.685z"
//   />,
//   "CheckBoxRounded"
// );

// const CheckBoxOutlineBlankIcon = createSvgIcon(
//   <path
//     transform="scale(0.7) translate(7,4)"
//     strokeWidth="1"
//     stroke="rgba(0, 0, 0, 0.54)"
//     d="M24 24h-24v-24h24v24zm-1-23h-22v22h22v-22zm-3"
//   />,
//   "CheckBoxOutlineBlankIcon"
// );

const Selections = observer(
  class Selections extends React.Component<any> {
    render() {
      let innerRef = this.props.innerRef;
      let onChange = this.props.onChange;
      let format = this.props.format;
      let checkedValues = this.props.value;
      let name = this.props.name;
      let disabled = this.props.hasOwnProperty("readonly")
        ? this.props.readonly
        : false;

      let listsState = this.props.listsState;
      let selects = listsState.get(this.props.selectData)
        ? listsState.get(this.props.selectData)
        : [];

      if (checkedValues == null) {
        checkedValues = [];
      }

      let checkboxes = selects.map(function (d: any) {
        let id = name + "-" + d.key;

        return (
          <FormControlLabel
            key={id}
            sx={{ marginLeft: "-6px", marginRight: "8px" }}
            control={
              <Checkbox
                color="primary"
                size="medium"
                icon={<TheCheckBoxOutlineBlank style={{ fontSize: 28 }} />}
                checkedIcon={<TheCheckboxIcon style={{ fontSize: 28 }} />}
                value={d.key}
                disabled={disabled}
                checked={checkedValues.includes(d.key)}
                inputProps={{ "aria-label": d.label }}
                onChange={onChange}
              />
            }
            label={d.label}
          ></FormControlLabel>
        );
      });
      return (
        <div ref={innerRef} className={format}>
          {checkboxes}
        </div>
      );
    }
  }
);

class Widget extends React.Component<any> {
  ref: any;

  constructor(props: any) {
    super(props);
    this.ref = createRef();
  }

  _onChange(event: any) {
    const key = this.props.value;

    let allVals: string[] = [];
    const dom = this.ref.current;
    Array.prototype.slice
      .call(dom.getElementsByTagName("input"))
      .forEach((el) => {
        if (el.checked) {
          allVals.push(el.value);
        }
      });
    WidgetUtils.setValue(
      this.props.model,
      key,
      allVals.length == 0 ? null : allVals
    );
    this.props.onChange(this.props.model);
  }

  clear() {
    WidgetUtils.setValue(this.props.model, this.props.value, null);
    this.props.onChange(this.props.model);
  }

  render() {
    let tooltip = WidgetUtils.tooltip(this.props, () => this.clear());
    //const name = this.props.name;
    let label: ReactElement = <></>;
    let value = WidgetUtils.dot(this.props.model, this.props.value);

    if (this.props.label) {
      label = (
        <IkformTypographyText>
          <span>{this.props.label}</span> {tooltip}
        </IkformTypographyText>
      );
    }

    let divClazz = "form-group";
    if (this.props.invalid) {
      divClazz = divClazz + " input-invalid";
    }

    return (
      <FormControl sx={{ gap: 0.5 }}>
        {label}
        <Selections
          innerRef={this.ref}
          readonly={this.props.readonly}
          name={this.props.name}
          onChange={(e: Event) => this._onChange(e)}
          value={value}
          listsState={listsState}
          selectData={this.props.selectData}
        />
      </FormControl>
    );
  }
}

export default Widget;
