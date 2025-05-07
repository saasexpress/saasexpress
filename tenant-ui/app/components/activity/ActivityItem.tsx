import BasicButton from "components/gold/basic-button";
import IkformTypographyText from "components/typography-text";
import {
  Avatar,
  Box,
  Stack,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  IconButton,
  Typography,
} from "@mui/material";
import { WarningOutlined, SearchOutlined } from "@mui/icons-material";

// import { FaExclamationTriangle } from "react-icons/fa";
// import { HiOutlineDocumentSearch } from "react-icons/hi";

export type ActivitySummary = {
  id: string;
  result: string;
  message: string;
  [params: string]: TemplateMap | string;
  activityAt: string;
};

export interface ActivitySortDate extends ActivitySummary {
  sortDate: string;
}

interface ActivityItemProps {
  data: ActivitySortDate;
  onSelect: (key: string, value: string) => void;
}

const ActivityItem: React.FC<ActivityItemProps> = ({ data, onSelect }) => {
  const compiled = template(data.message, data.params as TemplateMap);
  const regex = /(\{|<|\[|\]|>|\})/g;
  const clean = compact(compiled.split(regex));
  const text: any = [];

  clean.forEach((str: string, index: number, arr: any[]) => {
    if (!regex.test(str)) {
      switch (arr[index - 1]) {
        case "<":
          text.push(
            <Button
              key={uid(str)}
              data-filter-type="users"
              onClick={() => onSelect("users", str)}
            >
              {str}
            </Button>
          );
          break;
        case "{":
          text.push(
            <span
              key={uid(str)}
              data-filter-type="consumers"
              onClick={() => onSelect("consumers", str)}
            >
              {str}
            </span>
          );
          break;
        case "[":
          text.push(
            <IkformTypographyText key={uid(str)}>{str}</IkformTypographyText>
          );
          break;
        default:
          text.push(str);
          break;
      }
    }
  });

  return (
    <ActivityDetails
      data={data}
      text={text}
      // onOpen={onOpen}
      // onClose={onClose}
      // isOpen={isOpen}
    />
  );
};

export default ActivityItem;

// Adapted from just-template
// https://github.com/angus-c/just/blob/master/packages/string-template/index.js
type TemplateMap = {
  [key: string]: TemplateMap | string;
};

function template(string: string, data: TemplateMap): string {
  const proxyRegEx = /\{([^}]+)?\}/g;

  return string.replace(proxyRegEx, (_, key) => {
    const keyParts = key.split(".");
    let result = "";

    for (let i = 0; i < keyParts.length; i++) {
      if (!data) return "";

      switch (keyParts[i]) {
        case "Xactor":
          result += `<${data[keyParts[i]]}>`;
          break;
        case "Xconsumer":
          result += `{${data[keyParts[i]]}}`;
          break;
        case "Xaction":
          result += `[${data[keyParts[i]]}]`;
          break;
        default:
          result += data[keyParts[i]];
          break;
      }
    }

    return result || "";
  });
}

function compact(arg0: string[]) {
  // deduplicate using destructuring
  return [...new Set(arg0)];
}

// function to generate a uid
export function uid(str: string) {
  return str + Math.random().toString(36).substr(2, 9);
}

function ActivityDetails({ data, text, onOpen, onClose, isOpen }: any) {
  return (
    <Stack direction="row" alignItems="center" pb={5} data-content-id={data.id}>
      <Avatar
        {...stringAvatar(data.params?.actor)}
        sx={{ mr: 2, bgcolor: stringToColor(data.params?.actor) }}
      />
      <Box>
        <Stack direction="row" alignItems="center">
          {data.result === "failed" && (
            <IconButton size="small" sx={{ color: "error.main", mr: 1 }}>
              <WarningOutlined />
            </IconButton>
          )}
          <Typography>{text}</Typography>
          {data.blob && (
            <Box ml={2}>
              <Button
                startIcon={<SearchOutlined />}
                sx={{ color: "primary.main" }}
                onClick={onOpen}
              >
                More details
              </Button>
              <Dialog open={isOpen} onClose={onClose} maxWidth="xl" fullWidth>
                <DialogTitle>Activity Details</DialogTitle>
                <DialogContent>
                  {/* <YamlViewer doc={dump(data.blob)} /> */}
                  yaml viewer
                </DialogContent>
                <DialogActions>
                  <Button onClick={onClose}>Done</Button>
                </DialogActions>
              </Dialog>
            </Box>
          )}
        </Stack>
        <Typography
          variant="caption"
          color="text.secondary"
          component="time"
          dateTime={data.activityAt}
        >
          {new Date(data.activityAt).toLocaleTimeString("en-CA", {
            timeStyle: "short",
          })}
        </Typography>
      </Box>
    </Stack>
  );
}

function stringToColor(name: string) {
  let hash = 0;
  let i;

  if (!name) return "gray.100";

  /* eslint-disable no-bitwise */
  for (i = 0; i < name.length; i += 1) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash);
  }

  let color = "#";

  for (i = 0; i < 3; i += 1) {
    const value = (hash >> (i * 8)) & 0xff;
    color += `00${value.toString(16)}`.slice(-2);
  }
  /* eslint-enable no-bitwise */

  return color;
}

function stringAvatar(name: string) {
  if (!name) return { sx: { bgcolor: "transparent" }, children: "" };
  return {
    sx: {
      bgcolor: stringToColor(name),
    },
    children: `${name.split(" ")[0][0]}${
      name.split(" ").length > 1 ? name.split(" ")[1][0] : ""
    }`,
  };
}
