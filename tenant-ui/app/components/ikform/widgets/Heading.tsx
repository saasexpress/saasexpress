import React from "react";
import { Divider, FormControl, Typography } from "@mui/material";

import WidgetUtils from "../WidgetUtils";

class Heading extends React.Component<any> {
  render() {
    let tooltip = WidgetUtils.tooltip(this.props);
    const label = this.props.label;

    return (
      <FormControl key={label}>
        <Typography variant="h5">
          {label} {tooltip}
        </Typography>
        <Divider />
      </FormControl>
    );
  }
}

export default Heading;
