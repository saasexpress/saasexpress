import React, { FunctionComponent } from "react";

function CustomWidget(_type: string, Widget: FunctionComponent<any>) {
  return {
    isMatch: (type: string) => type === _type,
    createElement: function (config: any) {
      return React.createElement(Widget, config);
    },
  };
}

export default CustomWidget;
