import React, { useCallback, useRef, useState } from "react";

import Extensions from "@components/ikform/Extensions";
import FormRenderer from "@components/ikform/FormRenderer";
import { ListWatcher } from "components/ikform/ListStore";
import Code from "components/gold/code";

import PageAlert from "@lib/alerts/PageAlert";

// import FormBuilder from "@components/legacy/forms/widgets/FormBuilder.jsx";

import { Box, Button, Grid } from "@mui/material";
import MonacoEditor from "components/gold/monaco-editor";
import YAMLDialog from "components/gold/yaml-dialog";
import TagInput from "components/gold/tag-input";
import GoldHorizontalTabs from "components/gold/horizontal-tabs";

interface SettingsFormProps {
  id: string;
}

const SettingsForm = ({ id }: SettingsFormProps) => {
  var item = {};

  let ui = {
    selectedData: {
      add_choices: [
        { key: "compute", label: "Compute" },
        { key: "firewall", label: "Firewall" },
      ],
    },
    elements: [
      {
        type: "input",
        label: "Label",
        value: "label",
        help: "This is terrible!",
      },
      { type: "textarea", label: "Textarea", value: "label" },
      { type: "sensitive", label: "Sensitive", value: "label" },
      { type: "heading", label: "Heading" },
      { type: "toggle", label: "Toggle", value: "toggle" },
      { type: "datetime", label: "Datetime", value: "datetime" },
      { type: "relativedate", label: "Relative", value: "datetime" },
      {
        type: "checkbox",
        label: "Checkbox",
        value: "checkbox",
        selectData: "add_choices",
      },
      {
        type: "autocomplete",
        label: "Autocomplete",
        value: "autocomplete",
        selectData: "add_choices",
      },
      { type: "tagsinput", label: "Tags", value: "tags" },
      { type: "color", label: "Color", value: "color" },
      { type: "input", label: "Avatar", value: "avatar" },
      { type: "input", label: "Popularity", value: "popularity" },
    ],
  };

  const [model, setModel] = useState({
    model: { label: "Hello", datetime: new Date(), code: "code\n  other: yes" },
  });

  // let btn = false;
  // let btn = this.state.edit
  //   ? [
  //       <button className="btn btn-primary" onClick={() => this.toggleEdit()}>
  //         <i className="fas fa-cog" /> Cancel
  //       </button>,
  //       <button className="btn btn-default" onClick={() => this.save()}>
  //         Save
  //       </button>,
  //     ]
  //   : [
  //       <button className="btn btn-primary" onClick={() => this.toggleEdit()}>
  //         <i className="fas fa-cog" /> Change
  //       </button>,
  //     ];

  // const handleSelect = function (s) {
  //   return true;
  // };

  const handleChange = (model: any) => {
    setModel({ model });
  };

  const handleAction = (e: Event, a: string, d: any) => {
    console.log(e, a, d);
  };

  const ref = useRef(null);

  const cb = () => {
    PageAlert.doit({
      open: true,
      snackbar: true,
      title: "Page Alert",
      severity: "info",
      action: { link: "/shell", label: "Go to labels" },
      content: ["a", "b"],
    });
  };

  const setNewCode = (v: any) => {
    console.log(v);
    setModel({ model: { ...model.model, code: v } });
  };

  return (
    <Box pt={2}>
      <Box
        sx={{
          backgroundColor: "white",
          borderRadius: "3px",
          border: "thin solid #CCCCCC",
        }}
        p={1}
      >
        <Button onClick={cb}>Open page alert</Button>

        <Box>List Watcher</Box>
        <ListWatcher />

        <form className="smart-form">
          <FormRenderer
            id={id}
            extensions={Extensions}
            ref={ref}
            model={model.model}
            ui={ui}
            onChange={handleChange}
            onAction={handleAction}
          />
        </form>

        <Code>{JSON.stringify(model, null, 2)}</Code>

        <Grid
          container
          direction="row"
          justifyContent="center"
          alignItems="stretch"
          sx={{ minHeight: "400px" }}
          columns={{ xs: 8 }}
        >
          <Grid
            item
            p={1}
            xs={12}
            sx={{ border: "thin solid #CCCCCC", borderRadius: "5px" }}
          >
            <MonacoEditor
              defaultValue={model.model.code}
              language="yaml"
              options={{
                selectOnLineNumbers: true,
                roundedSelection: false,
                lineNumbers: "off",
                scrollBeyondLastLine: false,
                readOnly: false,
                theme: "vs",
                renderLineHighlight: "none",
                fontFamily: "Menlo, Consolas",
                fontSize: "14px",
                fontWeight: 600,
              }}
              onChange={(e: Event) => setNewCode(e)}
            />
          </Grid>
        </Grid>

        <YAMLDialog
          buttonLabel="Open YAML Dialog"
          title="Goody"
          onSubmit={() => false}
        >
          <Grid
            container
            direction="row"
            justifyContent="center"
            alignItems="stretch"
            sx={{ minHeight: "400px" }}
            columns={{ xs: 8 }}
          >
            <Grid
              item
              p={1}
              xs={12}
              sx={{ border: "thin solid #CCCCCC", borderRadius: "5px" }}
            >
              <MonacoEditor
                defaultValue={model.model.code}
                language="yaml"
                options={{
                  selectOnLineNumbers: true,
                  roundedSelection: false,
                  lineNumbers: "off",
                  scrollBeyondLastLine: false,
                  readOnly: false,
                  theme: "vs",
                  renderLineHighlight: "none",
                  fontFamily: "Menlo, Consolas",
                  fontSize: "14px",
                  fontWeight: 600,
                }}
                onChange={(e: Event) => setNewCode(e)}
              />
            </Grid>
          </Grid>
        </YAMLDialog>

        <TagInput />

        <GoldHorizontalTabs
          collection="blueprints"
          tabs={[
            { name: "details", label: "Details" },
            { name: "yaml", label: "YAML" },
            { name: "activity", label: "Activity" },
          ]}
          tab="details"
          onClick={(d: any) => {
            switch (d.name) {
            }
          }}
          params={{}}
        />

        {/* <div className="btn-toolbar btn-block">{btn}</div> */}

        {/* <form className="smart-form">
          <FormBuilder
            data={item}
            ui={ui}
            onChange={onChange}
            onAction={handleAction}
          />
        </form> */}
      </Box>
    </Box>
  );
};

export default SettingsForm;
