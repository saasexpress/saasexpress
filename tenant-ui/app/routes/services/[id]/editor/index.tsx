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
import { useEffect, useRef, useState } from "react";
import Close from "components/gold/close";
import BasicButton from "components/gold/basic-button";
import GoldHorizontalTabs from "components/gold/horizontal-tabs";
import Code from "components/gold/code";
import { Node } from "components/graph";
import JsonData from "components/gold/json-data";
import MonacoEditor from "components/gold/monaco-editor";
import { parse, stringify } from "yaml";
import { useSearchParams } from 'react-router-dom';
import { useAppContext } from "context";
import SubHeader from "../sub-header";

const Entry = () => {
  const { id } = useParams();
  const { search } = useAppContext();

  const q_variant = search.get("variant") || "";

  const theme = useTheme();
  const isSmall = useMediaQuery(theme.breakpoints.down("sm"));

  const api = useAPIClient();

  const container = useRef(null);

  const [showDetail, setShowDetail] = useState(false);
  const [list, setList] = useState(12);

  const [selectedNode, setSelected] = useState<Node>();

  const [variant, setVariant] = useState(q_variant);

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

  useEffect(() => {
    if (data?.data && q_variant == "") {
      if (Object.keys(data?.data.variants).length == 0) {
        setVariant("V1");
        return;
      }
      setVariant(Object.keys(data?.data.variants).sort().reverse().pop() as string);
    }
  }, [data]);

  const setNewCode = (code: string) => {
    console.log(code);
  };

  return (
    <Container maxWidth="lg">
      <Header name={d?.displayName} id={d?.id} />
      {data?.data?.variants && <SubHeader serviceId={d?.id} variantId={variant} variants={data?.data.variants} />}
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
            title={"Integration" + ` ${data?.data.variants[variant]?.dag.name}`}
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
              {data?.data && (
                <DynamicGraph
                  key={id + variant}
                  id={id}
                  data={data?.data}
                  variant={variant}
                  onSelected={(node: Node) => {
                    setSelected(node);
                    setShowDetail(true);
                    setList(4);
                  }}
                />
              )}
            </Grid2>
            <Grid2 size={{ md: 8 }} pl={0}>
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
                          ? [
                              { name: "yaml", label: "YAML" },
                              { name: "in", label: "Inputs" },
                              { name: "out", label: "Outputs" },
                            ]
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
                          defaultValue={stringify(
                            selectedNode?.config,
                            null,
                            4
                          )}
                          language="yaml"
                          theme="vs-dark"
                          onChange={setNewCode}
                          options={{
                            automaticLayout: true,
                            autoIndent: true,
                            cursorStyle: "line",
                            fontFamily: "-apple-system, Menlo, Consolas",
                            fontSize: "14px",
                            fontWeight: 400,
                            foldingHighlight: false,
                            formatOnPaste: true,
                            lineNumbers: true,
                            minimap: { enabled: false },
                            occurrencesHighlight: false,
                            readOnly: false,
                            renderLineHighlight: "none",
                            roundedSelection: false,
                            scrollBeyondLastLine: false,
                            selectOnLineNumbers: false,
                            showFoldingControls: "mouseover",
                            tabIndex: 2,
                            wordWrap: false,
                          }}
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
