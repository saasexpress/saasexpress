import {
  Box,
  Container,
  Grid,
  Grid2,
  IconButton,
  Paper,
  Slide,
  Stack,
  Typography,
  useMediaQuery,
  useTheme,
} from "@mui/material";
import { useParams } from "react-router";
import Header from "../header";
import Nav from "../nav";

import DetailHeader from "@components/detail-header";
import useAPIClient from "lib/api/APIClient";
import DynamicGraph from "./graph";
import { useRef, useState } from "react";
import Close from "components/gold/close";
import BasicButton from "components/gold/basic-button";
import GoldHorizontalTabs from "components/gold/horizontal-tabs";
import Code from "components/gold/code";
import { Node } from "components/graph";
import JsonData from "components/gold/json-data";
import MonacoEditor from "components/gold/monaco-editor";
import { parse, stringify } from "yaml";

const Entry = () => {
  const { id } = useParams();
  const theme = useTheme();
  const isSmall = useMediaQuery(theme.breakpoints.down("sm"));

  const api = useAPIClient();

  const container = useRef(null);

  const [showDetail, setShowDetail] = useState(false);
  const [list, setList] = useState(12);

  const [selectedNode, setSelected] = useState<Node>();

  if (!id) {
    return <></>;
  }

  const { isPending, data } = api.get(["service", id], `/api/services/${id}`, {
    refetchOnMount: true,
  });

  // if (isPending) {
  //   return <></>;
  // }

  const d = data?.data;

  const setNewCode = (code: string) => {
    alert("new " + code);
  };
  return (
    <Container maxWidth="lg">
      <Header name={d?.displayName} id={d?.id} />

      <Grid2
        key={d?.id}
        container
        direction="row"
        spacing={3}
        alignItems="top"
        mt={1}
      >
        {!isSmall && (
          <Grid2>
            <Nav tab="editor" params={{ id: d?.id }} />
          </Grid2>
        )}
        <Grid2 flex={1}>
          <DetailHeader
            title="Integration"
            description="Editor for changing the integration flow"
            actions={isSmall && <Nav tab="editor" params={{ id: d?.id }} />}
          />
          <Grid2
            ref={container}
            container
            justifyContent="space-between"
            alignItems="flex-start"
            mt={0}
            mb={5}
            spacing={2}
          >
            <Grid2 size={{ md: list }} overflow="clip">
              <DynamicGraph
                id={id}
                data={data?.data}
                variant={"activity-api-mutation.yaml"}
                onSelected={(node: Node) => {
                  setSelected(node);
                  setShowDetail(true);
                  setList(6);
                }}
              />
            </Grid2>
            <Grid2 size={{ md: 6 }} pl={0}>
              <Slide
                direction="left"
                in={showDetail}
                mountOnEnter
                unmountOnExit
                container={container?.current}
                easing={{
                  enter: "sharp",
                  exit: "linear",
                }}
                onExited={() => {
                  setList(12);
                }}
              >
                <Paper
                  elevation={0}
                  sx={{ border: "1px solid #CCCCCC", borderRadius: "3px" }}
                >
                  <Box
                    component="form"
                    noValidate
                    p={0}
                    sx={{
                      backgroundColor: "white",
                      borderRadius: "0",
                    }}
                    border="10px solid white"
                  >
                    <Stack
                      direction="row"
                      alignItems="flex-start"
                      justifyContent="flex-start"
                      // sx={{ borderBottom: 'thin solid #CCCCCC' }}
                      p={1}
                    >
                      <Grid2>
                        <Typography variant="h4">
                          {selectedNode?.action} ({selectedNode?.label})
                        </Typography>
                      </Grid2>
                      <Grid2 flex={1} textAlign="right" mr={0} p={0}>
                        <Stack direction="row" justifyContent="right">
                          <IconButton onClick={() => setShowDetail(false)}>
                            <Close />
                          </IconButton>
                        </Stack>
                      </Grid2>
                    </Stack>

                    <GoldHorizontalTabs
                      key={id}
                      collection="services"
                      tabs={
                        id
                          ? [{ name: "yaml", label: "YAML" }]
                          : [{ name: "yaml", label: "Details" }]
                      }
                      tab="yaml"
                      onClick={(d: any) => {
                        switch (d.name) {
                          default:
                            return false;
                        }
                      }}
                      params={{}}
                    />
                  </Box>
                  <Box>
                    <Grid2
                      container
                      direction="row"
                      justifyContent="center"
                      alignItems="stretch"
                      sx={{ minHeight: "400px" }}
                      columns={{ xs: 8 }}
                    >
                      <Grid2 p={1} size={{ xs: 12 }}>
                        <MonacoEditor
                          key={selectedNode?.id}
                          defaultValue={stringify(selectedNode, null, 4)}
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
                          onChange={setNewCode}
                        />
                      </Grid2>
                    </Grid2>
                  </Box>
                </Paper>
              </Slide>
            </Grid2>
          </Grid2>
        </Grid2>
      </Grid2>
    </Container>
  );
};

export default Entry;
