import React from "react";

import WidgetUtils from "../WidgetUtils";

import {
  IntlProvider,
  FormattedDate,
  FormattedTime,
  FormattedNumber,
  FormattedPlural,
} from "react-intl";
import { FormControl, Typography } from "@mui/material";
import IkformTypographyText from "@components/typography-text";

class DateTime extends React.Component<any> {
  _onChange(event: any) {
    const key = this.props.value;
    WidgetUtils.setValue(this.props.model, key, event.target.value);

    this.props.onChange(this.props.model);
  }

  render() {
    //  ui-autocomplete-loading

    let tooltip = WidgetUtils.tooltip(this.props);
    const name = this.props.name;
    let label = <></>;
    let value = WidgetUtils.dot(this.props.model, this.props.value);

    if (this.props.label) {
      label = (
        <IkformTypographyText>
          {this.props.label} {tooltip}
        </IkformTypographyText>
      );
    }

    return (
      <FormControl sx={{ gap: 0.75 }} key={name}>
        {label} {tooltip}
        <IkformTypographyText>
          <FormattedDate
            value={new Date(value)}
            year="numeric"
            day="2-digit"
            month="short"
          />{" "}
          <FormattedTime value={new Date(value)} />
        </IkformTypographyText>
      </FormControl>
    );
  }
}

export default DateTime;
