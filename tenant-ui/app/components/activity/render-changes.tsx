import Arrow from "@components/icons/svg-arrow";
import { Stack, Typography } from "@mui/material";

function safe(v: any) {
  if (v == null) {
    return "";
  } else if (typeof v !== "string" && typeof v !== "number") {
    return JSON.stringify(v, null, 3);
  } else {
    return v;
  }
}

const renderChanges = (activity: any) => {
  let ignore = ["etag", "lastChangeDate"];

  let version = "";
  let count = 0;

  const changes = activity.changes.map((c: any) => {
    console.log(JSON.stringify(c, null, 3));
    let key = c.globalId.fragment;

    let property = key
      ? key.replace(new RegExp("_children/", "g"), "")
      : c.property;

    if (c.changeType == "MapChange") {
      let subChanges = c.entryChanges.map((s: any) => {
        count++;
        if (s.entryChangeType == "EntryAdded") {
          return (
            <Stack direction="row">
              <Typography
                //variant="ikform"
                sx={{ minWidth: "200px", pr: "5px" }}
              >
                {s.key}
              </Typography>
              <Typography>
                <Arrow /> {safe(s.value)}
              </Typography>
            </Stack>
          );
        } else if (s.entryChangeType == "EntryValueChange") {
          return (
            <Stack direction="row">
              <Typography sx={{ minWidth: "200px", pr: "5px" }}>
                {s.key}
              </Typography>
              <Typography>
                {"" + safe(s.leftValue)}
                <Arrow /> {"" + safe(s.rightValue)}
              </Typography>
            </Stack>
          );
        } else {
          return false;
        }
      });
      return <Stack direction="column">{subChanges}</Stack>;
    } else if (property == "version") {
      version = c.right;
    } else if (c.changeType == "ObjectRemoved") {
      count++;
      return (
        <div className="activity-item-value">
          <span className="activity-field">{property}</span>
          <span className="activity-detail">
            <Arrow /> REMOVED
          </span>
        </div>
      );
    } else if (c.changeType == "NewObject") {
    } else if (c.changeType == "ListChange") {
      count++;
      let chgs = c.elementChanges.map((h: any) => {
        let ic =
          h.elementChangeType == "ValueAdded" ? (
            <i className="fas fa-lg fa-plus-square txt-color-gray"></i>
          ) : (
            <i className="fas fa-lg fa-minus-square txt-color-gray"></i>
          );
        return (
          <Typography>
            {ic} {safe(h.value)}
          </Typography>
        );
      });

      return (
        <Stack direction="column">
          <Typography>
            {property == "content" ? c.property : property}
          </Typography>
          {chgs}
        </Stack>
      );
    } else if (ignore.indexOf(property) == -1) {
      /* ValueChange */
      console.log("Returning Stack");
      count++;
      let prop = property.startsWith("content")
        ? property.substring("content".length)
        : property;

      return (
        <Stack direction="row">
          <Typography sx={{ minWidth: "200px", pr: "5px" }}>
            {prop} {prop == c.property ? "" : c.property}
          </Typography>
          <Typography>
            {safe(c.left)} <Arrow /> {safe(c.right)}
          </Typography>
        </Stack>
      );
    }
    return false;
  });

  return { changes, version, count };
};

export default renderChanges;
