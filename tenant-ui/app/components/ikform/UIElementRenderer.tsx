import React, { ReactNode } from "react";

import WidgetUtils from "./WidgetUtils";

import {
  Box,
  Divider,
  FormControl,
  Grid2,
  Stack,
  Typography,
} from "@mui/material";
import IkformTypographyText from "@components/typography-text";

//import * as clazz from './UIElements.scss'
// import Input from './Input.jsx';
// // import Heading from './Heading.jsx'
// import RadioGroup from './RadioGroup.jsx';
// import Textarea from './Textarea.jsx';
// import Checkbox from './Checkbox.jsx';
// import Panel from './Panel.jsx';
// import Actions from './Actions.jsx';
// // import NestableList from './NestableList.jsx'
// import Table from './Table.jsx';
// import ResultSet from './ResultSet.jsx';
// // import Select from './Select.jsx'
// import Text from './Text.jsx';
// import Sensitive from './Sensitive.jsx';
// import DateTime from './DateTime.jsx';
// import RelativeDate from './RelativeDate.jsx';
// import KeyData from './KeyData.jsx';
// import KeyPair from './KeyPair.jsx';
// // import Logo from './Logo.jsx'
// import Tags from './Tags.jsx';
// // import Toggle from './Toggle.jsx'
// import Label from './Label.jsx';
// import ColorPicker from './ColorPicker.jsx';
// // import AbstractImage from './AbstractImage.jsx'
// // import CodeEditor from './CodeEditor.jsx'
// import SelectedFilter from './SelectedFilter.jsx';
// import IkformTypographyText from '../../cards/shared/typography-text';

function addConfig(newConfig: any, config: any, keys: string[]) {
  keys.map(function (key: string) {
    if (config.hasOwnProperty(key)) {
      newConfig[key] = config[key];
    }
  });
}

interface UIElementRendererProps {
  id?: string;
  model: any;
  ui: any;
  onChange: any;
  onAction: any;
  extensions: any;
  parentId?: string;
  parent?: any;
  key?: string;
  emitter: any;
  mode: string;
}

const UIElementRenderer = ({
  extensions,
  mode,
  model,
  parent,
  parentId,
  key: propKey,
  id,
  ui,
  emitter,
  onChange,
  onAction,
}: UIElementRendererProps) => {
  const key = propKey ? propKey : id;

  // let counter = 0;

  let elements = ui.map(function (_config: any, index: number) {
    const config = { ..._config };
    config.model = model;
    config.key = "k_" + (key ? key + "_" : "") + config.name + "_" + index;
    config.onChange = onChange;
    config.onAction = onAction;
    config.emitter = emitter;
    config.extensions = extensions;
    config.mode = mode;
    //console.log("UIElementRenderer / Render " + config.key);
    //console.log(JSON.stringify(config));
    let element = null;
    if (false) {
      // if (
      //   config.type == "input" ||
      //   config.type == "password" ||
      //   config.type == "email"
      // ) {
      //   element = React.createElement(Input, config);
      // } else if (config.type == "text") {
      //   element = React.createElement(Text, config);
      //   // } else if (config.type == "text") {
      //   //     element = React.createElement(Text, config);
      // } else if (config.type == "keypair") {
      //   element = React.createElement(KeyPair, config);
      // } else if (config.type == "relativedate") {
      //   element = React.createElement(RelativeDate, config);
      // } else if (config.type == "radiogroup") {
      //   element = React.createElement(RadioGroup, config);
      // } else if (config.type == "actions") {
      //   element = React.createElement(Actions, config);
      //   // } else if (config.type == "select") {
      //   //     element = React.createElement(Select, config);
      //   // } else if (config.type == "toggle") {
      //   //     element = React.createElement(Toggle, config);
      // } else if (config.type == "color") {
      //   element = React.createElement(ColorPicker, config);
      //   // } else if (config.type == "logo") {
      //   //     element = React.createElement(Logo, config);
      // } else if (config.type == "label") {
      //   element = React.createElement(Label, config);
      //   // } else if (config.type == "abstract") {
      //   //     element = React.createElement(AbstractImage, config);
      // } else if (config.type == "tags") {
      //   element = React.createElement(Tags, config);
      // } else if (config.type == "sensitive") {
      //   element = React.createElement(Sensitive, config);
      // } else if (config.type == "textarea") {
      //   element = React.createElement(Textarea, config);
      // } else if (config.type == "datetime") {
      //   element = React.createElement(DateTime, config);
      // } else if (config.type == "checkbox") {
      //   element = React.createElement(Checkbox, config);
      // } else if (config.type == "codeeditor") {
      //     element = React.createElement(CodeEditor, config);
      // } else if (config.type == "selectedfilter") {
      //     element = React.createElement(SelectedFilter, config);
      // } else if (config.type == "heading") {
      //     element = React.createElement(Heading, config);
      // } else if (config.type == "nestablelist") {
      //     element = React.createElement(NestableList, config);
      // } else
      // } else if (config.type == "selectedfilter") {
      //   element = React.createElement(SelectedFilter, config);
    } else if (config.type == "section") {
      element = React.createElement(UIElementRenderer, {
        model: config.model,
        mode: config.mode,
        ui: config.elements,
        onChange: config.onChange,
        onAction: config.onAction,
        extensions: config.extensions,
        emitter: config.emitter,
      });
      let style = config.hasOwnProperty("style")
        ? config.style
        : "ikform-flex-column";

      const tooltip = WidgetUtils.tooltip(config);

      return (
        <FormControl key={config.key}>
          <Typography variant="h5">
            {config.label} {tooltip}
          </Typography>
          <Divider />
          <Stack direction="row" spacing={2} justifyContent="space-evenly">
            {element}
          </Stack>
        </FormControl>
      );
    } else if (config.type == "row") {
      let middle = config.elements.map(function (c: any) {
        let element = React.createElement(UIElementRenderer, {
          model: config.model,
          mode: config.mode,
          ui: [c],
          onChange: config.onChange,
          onAction: config.onAction,
          extensions: config.extensions,
          emitter: config.emitter,
        });
        return <Grid2>{element}</Grid2>;
      });
      let label: ReactNode = undefined;
      if (config.label) {
        label = <IkformTypographyText>{config.label}</IkformTypographyText>;
      }
      return (
        <section key={config.key}>
          {label}
          <Grid2
            container
            direction="row"
            justifyContent="space-evenly"
            alignItems="flex-start"
            rowSpacing={0}
            columnSpacing={{ xs: 1, sm: 2, md: 3 }}
          >
            {middle}
          </Grid2>
        </section>
      );
    } else if (config.type == "panelgroup") {
      //counter = 0;

      element = React.createElement(UIElementRenderer, {
        model: config.model,
        mode: config.mode,
        parent: config,
        parentId: config.name,
        ui: config.elements,
        onChange: config.onChange,
        onAction: config.onAction,
        extensions: config.extensions,
        emitter: config.emitter,
      });
      return (
        <Box
          component="div"
          key={config.name}
          id={config.name}
          sx={{ backgroundColor: "inherit" }}
        >
          {element}
        </Box>
      );
      // } else if (config.type == "table") {
      //   counter = 0;
      //   element = React.createElement(Table, {
      //     key: config.key,
      //     model: config.model,
      //     ui: {
      //       dragdrop: config.dragdrop,
      //       stringarray: config.stringarray,
      //       elements: config.elements,
      //     },
      //     value: config.value,
      //     label: config.label,
      //     onChange: config.onChange,
      //     extensions: config.extensions,
      //   });
      //   return element;
      // } else if (config.type == "keydata") {
      //   counter = 0;
      //   let newConfig = {
      //     key: config.key,
      //     model: config.model,
      //     ui: config.fields,
      //   };
      //   self.addConfig(newConfig, config, [
      //     "value",
      //     "help",
      //     "uiSelector",
      //     "label",
      //     "onChange",
      //     "onAction",
      //     "emitter",
      //     "mode",
      //     "extensions",
      //     "keyMatch",
      //     "depth",
      //   ]);

      //   element = React.createElement(KeyData, newConfig);
      //   return element;
      // } else if (config.type == "resultset") {
      //   counter = 0;
      //   let newConfig = {
      //     rkey: config.key,
      //     ui: config.elements,
      //     header: true,
      //     paging: false,
      //     numItemsPerPage: 25,
      //     search: false,
      //     selectable: false,
      //   };
      //   addConfig(newConfig, config, [
      //     "key",
      //     "extensions",
      //     "name",
      //     "label",
      //     "model",
      //     "value",
      //     "cssClass",
      //     "onChange",
      //     "onAction",
      //     "paging",
      //     "search",
      //     "selectable",
      //     "header",
      //     "numItemsPerPage",
      //     "selectedItem",
      //     "readonly",
      //     "drilldown",
      //   ]);

      //   element = React.createElement(ResultSet, newConfig);
      //   return element;
      // } else if (config.type == "panel") {
      //   let newConfig = {
      //     id: parentId + "_Panel_" + counter,
      //     index: index,
      //     parentId: parentId,
      //     style: parent?.style,
      //     ui: config.elements,
      //     mode: config.mode,
      //   };
      //   addConfig(newConfig, config, [
      //     "label",
      //     "help",
      //     "defaultExpanded",
      //     "model",
      //     "onChange",
      //     "onAction",
      //     "extensions",
      //   ]);

      //   element = React.createElement(Panel, newConfig);
      //   counter++;
    } else if (extensions) {
      let answer = extensions
        .filter((e: any) => e.isMatch(config.type))
        .map((e: any) => e.createElement(config));
      if (answer.length == 0) {
        console.log("UIRenderer Extensions / No match for " + config.type);
      }
      return answer.length == 1 ? (
        answer[0]
      ) : (
        <p key={"invalid-" + config.key}>
          INVALID {"invalid-" + config.key} {config.type} ({answer.length})
        </p>
      );
    } else {
      console.log("UIRenderer / No match for " + config.type);
      //                element =(<p>INVALID</p>)
      // element = React.createElement(Text, config);
    }
    return element;
  });
  //return <Box>Good</Box>;
  if (elements.length == 1) {
    //console.log("UIRenderer / Return single element");
    let element = elements[0];
    return element;
  }
  return elements;
};

export default UIElementRenderer;
