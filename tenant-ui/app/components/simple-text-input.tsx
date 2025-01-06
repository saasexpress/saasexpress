import React, { useEffect } from "react";
// import { makeStyles } from "@mui/styles";
import TextField from "@mui/material/TextField";
import styled from "@emotion/styled";

// const AntTabs = styled((props: any) => <Tabs {...props} />)({

// const useStyles = makeStyles(() => ({
//   margin: {
//     margin: 1,
//     marginBottom: 5,
//     display: "flex",
//     borderRadius: 0,
//     backgroundColor: "white",
//   },
//   input: {},
// }));

export default function SimpleTextInput(props: any) {
  //const classes = useStyles();
  const [search, setSearch] = React.useState(props.search);
  const [clear, setClear] = React.useState(false);

  useEffect(() => {
    setClear(search && search.length > 0);
  });

  const handleChange = (event: any) => {
    setSearch(event.target.value);
    props.onChange(event.target.value);
  };

  const handleClear = (event: any) => {
    setSearch("");
    props.onChange("");
  };

  return (
    <TextField
      variant="outlined"
      autoComplete="off"
      fullWidth
      inputRef={(input) => {
        if (input != null) {
          input.focus();
        }
      }}
      value={search}
      name={props?.name}
      placeholder={props?.placeholder}
      onChange={handleChange}
      {...props}
    />
  );
}
