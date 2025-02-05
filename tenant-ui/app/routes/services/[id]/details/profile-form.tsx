import { useCallback, useMemo, useRef, useState } from "react";
import { Box, Button } from "@mui/material";

import Extensions from "@components/ikform/Extensions";
import FormRenderer from "@components/ikform/FormRenderer";
import useAPIClient from "lib/api/APIClient";
import { useQueryClient } from "@tanstack/react-query";
import APIErrorHandler from "lib/alerts/APIErrorHandler";

interface Service {
  id: string;
  displayName: string;
}

interface ProfileFormProps {
  item: Service;
}

const ProfileForm = ({ item }: ProfileFormProps) => {
  const queryClient = useQueryClient();

  let ui = {
    elements: [
      {
        type: "input",
        label: "ID",
        value: "id",
        readonly: true,
      },
      {
        type: "input",
        label: "Display Name",
        value: "displayName",
        readonly: false,
      },
    ],
  };

  // important to clone the item on initialize so that
  // the invalidateQueries works properly on change of data
  const [model, setModel] = useState({
    model: { ...item },
  });

  const handleChange = (model: any) => {
    setModel({ model });
  };

  const handleAction = (e: Event, a: string, d: any) => {
    console.log(e, a, d);
  };

  const ref = useRef(null);

  const api = useAPIClient();

  const save = useCallback((ev: any) => {
    ev.preventDefault();

    api.put(`/api/services/${item.id}`, model.model, () => {
      console.log("dispatched", item.id);
      queryClient.invalidateQueries({ queryKey: ["service", item.id] });
      queryClient.invalidateQueries({ queryKey: ["services"] });
      queryClient.invalidateQueries({ queryKey: ["list-activity"] });
      setMode("ro");
      APIErrorHandler.notice({
        title: "Service",
        content: "Updated successfully",
      });
    });
  }, []);

  const [mode, setMode] = useState("ro");

  let btn =
    mode === "edit"
      ? [
          <Button key="cancel" onClick={() => setMode("ro")}>
            Cancel
          </Button>,
          <Button key="save" type="submit" onClick={save}>
            Save
          </Button>,
        ]
      : [
          <Button key="change" onClick={() => setMode("edit")}>
            Change
          </Button>,
        ];

  ui.elements.forEach((ui) => (ui.readonly = mode === "edit" ? false : true));

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
        <form>
          {btn}
          <FormRenderer
            id={item.id}
            extensions={Extensions}
            ref={ref}
            model={model.model}
            ui={ui}
            onChange={handleChange}
            onAction={handleAction}
          />
        </form>
      </Box>
    </Box>
  );
};

export default ProfileForm;
