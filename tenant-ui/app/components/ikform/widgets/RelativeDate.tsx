import WidgetUtils from "../WidgetUtils";

//import { OverlayTrigger, Tooltip, Popover } from "react-bootstrap";
import TimeAgo from "react-timeago";

import { FormattedDate, FormattedTime } from "react-intl";

import ToolTipSimple from "@components/gold/tooltip-simple";
import IkformTypographyText from "@components/typography-text";
import { FormControl } from "@mui/material";

const RelativeDate = (props: any) => {
  // _onChange(event: any) {
  //   const key = props.value;
  //   WidgetUtils.setValue(props.model, key, event.target.value);

  //   props.onChange(props.model);
  // }

  //  ui-autocomplete-loading

  let tooltip = WidgetUtils.tooltip(props);
  const name = props.name;
  let label = <></>;
  let value = WidgetUtils.dot(props.model, props.value);

  if (props.label) {
    label = (
      <IkformTypographyText>
        {props.label} {tooltip}
      </IkformTypographyText>
    );
  }

  let body = (
    <ToolTipSimple
      placement="bottom"
      tooltip={
        <span>
          <FormattedDate
            value={new Date(value)}
            year="numeric"
            day="2-digit"
            month="short"
          />{" "}
          <FormattedTime value={new Date(value)} />
        </span>
      }
    >
      <span>
        <TimeAgo title={""} date={new Date(value)} minPeriod={60} />
      </span>
    </ToolTipSimple>
  );

  return (
    <FormControl sx={{ gap: 0.75 }} key={name}>
      {label}
      <IkformTypographyText>{body}</IkformTypographyText>
    </FormControl>
  );
};

export default RelativeDate;
