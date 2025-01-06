import { Cancel } from "@mui/icons-material";
import { Box, Stack, TextField, Typography } from "@mui/material";
import { useRef, useState } from "react";

const Tags = ({ data, handleDelete }: any) => {
  return (
    <Box
      sx={{
        background: "#283240",
        height: "100%",
        display: "flex",
        padding: "0.4rem",
        margin: "0 0.5rem 0 0",
        justifyContent: "center",
        alignContent: "center",
        color: "#ffffff",
      }}
    >
      <Stack direction="row" gap={1}>
        <Typography>{data}</Typography>
        <Cancel
          sx={{ cursor: "pointer" }}
          onClick={() => {
            handleDelete(data);
          }}
        />
      </Stack>
    </Box>
  );
};

export default function TagInput() {
  const [tags, setTags] = useState<any>([]);
  const tagRef = useRef(null as any);

  const handleDelete = (value: any) => {
    const newtags = tags.filter((val: any) => val !== value);
    setTags(newtags);
  };
  const handleOnSubmit = (e: any) => {
    e.preventDefault();
    setTags([...tags, tagRef.current.value]);
    tagRef.current.value = "";
  };
  return (
    <Box sx={{ flexGrow: 1 }}>
      <form onSubmit={handleOnSubmit}>
        <TextField
          inputRef={tagRef}
          fullWidth
          variant="standard"
          size="small"
          sx={{ margin: "1rem 0" }}
          margin="none"
          placeholder={tags.length < 5 ? "Enter tags" : ""}
          InputProps={{
            startAdornment: (
              <Box sx={{ margin: "0 0.2rem 0 0", display: "flex" }}>
                {tags.map((data: any, index: number) => {
                  return (
                    <Tags data={data} handleDelete={handleDelete} key={index} />
                  );
                })}
              </Box>
            ),
          }}
        />
      </form>
    </Box>
  );
}
