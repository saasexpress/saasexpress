import React, { useState } from "react";
import { TextField, IconButton, Typography, Box, Button } from "@mui/material";
import { Edit as EditIcon } from "@mui/icons-material";

const InPlaceEdit = ({ initialValue = "", onSave }: any) => {
  const [value, setValue] = useState(initialValue); // The current value being displayed.
  const [editValue, setEditValue] = useState(initialValue); // The value being edited.
  const [isEditing, setIsEditing] = useState(false);

  const handleEdit = () => {
    setEditValue(value); // Start with the current value when editing begins.
    setIsEditing(true);
  };

  const handleSave = () => {
    setValue(editValue); // Save the edited value.
    setIsEditing(false);
    if (onSave) onSave(editValue); // Trigger the onSave callback if provided.
  };

  const handleCancel = () => {
    setEditValue(value); // Revert to the original value.
    setIsEditing(false);
  };

  const handleChange = (event: any) => {
    setEditValue(event.target.value);
  };

  return (
    <Box display="flex" alignItems="center" gap={1}>
      {isEditing ? (
        <>
          <TextField
            value={editValue}
            onChange={handleChange}
            size="small"
            autoFocus
            variant="outlined"
            fullWidth
          />
          <Button
            size="small"
            color="primary"
            variant="contained"
            onClick={handleSave}
          >
            Save
          </Button>
          <Button
            size="small"
            color="secondary"
            variant="outlined"
            onClick={handleCancel}
          >
            Cancel
          </Button>
        </>
      ) : (
        <>
          <Typography variant="h3" noWrap>
            {value}
          </Typography>
          <IconButton size="small" onClick={handleEdit} aria-label="Edit">
            <EditIcon />
          </IconButton>
        </>
      )}
    </Box>
  );
};

export default InPlaceEdit;
