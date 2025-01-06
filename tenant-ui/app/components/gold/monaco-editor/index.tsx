/*
'use client';
import { Box } from '@mui/material';
import * as monaco from 'monaco-editor';
import { useEffect, useRef } from 'react';

export default function MonacoEditor({ options }) {
  const editorRef = useRef(null);

  useEffect(() => {
    if (typeof window == 'undefined') {
      return;
    }

    const editorInstance = monaco.editor.create(editorRef.current, options);

    const handleChange = () => {
      options.onChange(editorInstance.getValue());
    };

    editorInstance.onDidChangeModelContent(handleChange);

    return () => {
      editorInstance.dispose();
    };
  }, []);

  return <Box sx={{ minHeight: '100px', height: '100%' }} ref={editorRef} />;
}
*/
import Editor from "@monaco-editor/react";
import { Box } from "@mui/material";

export default function MonacoEditor(props: any) {
  // if (typeof window !== 'undefined') {
  //   monaco.init().then((monaco) => {
  //     console.log(monaco);
  //   });
  // }
  return (
    <Box sx={{ minHeight: "100px", height: "100%" }}>
      <Editor height="100%" {...props} />
    </Box>
  );
}
