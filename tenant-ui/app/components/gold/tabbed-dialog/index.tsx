import React from "react";
//import FormRenderer from '@/ikform/forms/FormRenderer';
import {
  Box,
  Button,
  Card,
  CardContent,
  CardActions,
  CardHeader,
  CardMedia,
  Grid,
  Stack,
  Paper,
  Tabs,
  Tab,
} from "@mui/material";

import Avatar from "@mui/material/Avatar";
import IconButton, { IconButtonProps } from "@mui/material/IconButton";
import { red } from "@mui/material/colors";
import CloseIcon from "@mui/icons-material/Close";

//import { withStyles } from "@mui/styles";
import styled from "@emotion/styled";

import Backdrop from "@mui/material/Backdrop";
import Modal from "@mui/material/Modal";
import Fade from "@mui/material/Fade";
import Typography from "@mui/material/Typography";

// const Item = styled(Paper)(({ theme }) => ({
//   padding: theme.spacing(10),
//   textAlign: 'left',
//   width: '100%',
//   color: theme.palette.text.primary,
// }));

const style = {
  position: "absolute" as "absolute",
  top: "50%",
  left: "50%",
  transform: "translate(-50%, -50%)",
  width: "100%",
  maxWidth: 700,
  bgcolor: "background.paper",
  border: "0 solid #000",
  boxShadow: 24,
  p: 0,
  m: 0,
};

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`simple-tabpanel-${index}`}
      aria-labelledby={`simple-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ p: 3 }}>
          <Typography>{children}</Typography>
        </Box>
      )}
    </div>
  );
}

function a11yProps(index: number) {
  return {
    id: `simple-tab-${index}`,
    "aria-controls": `simple-tabpanel-${index}`,
  };
}

const AntTabs = styled((props: any) => <Tabs {...props} />)({
  root: {
    borderBottom: "1px solid #e8e8e8",
    flexGrow: 1,
  },
  indicator: {
    backgroundColor: "primary.main",
  },
});

const AntTab = styled((props: any) => <Tab {...props} />)({
  root: {
    textTransform: "none",
    minWidth: 72,
    fontWeight: "typography.fontWeightRegular",
    marginRight: 4,
    color: "primary.main",
    fontFamily: [
      "-apple-system",
      "BlinkMacSystemFont",
      '"Segoe UI"',
      "Roboto",
      '"Helvetica Neue"',
      "Arial",
      "sans-serif",
      '"Apple Color Emoji"',
      '"Segoe UI Emoji"',
      '"Segoe UI Symbol"',
    ].join(","),
    "&:hover": {
      color: "primary.main",
      opacity: 1,
    },
    "&$selected": {
      color: "primary.main",
      fontWeight: "typography.fontWeightMedium",
    },
    "&:focus": {
      color: "primary.main",
    },
  },
  selected: {},
});

export default function ModalDialog() {
  const [open, setOpen] = React.useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);

  const [value, setValue] = React.useState(0);

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <div>
      <Button onClick={handleOpen}>Open modal</Button>
      <Modal
        aria-labelledby="transition-modal-title"
        aria-describedby="transition-modal-description"
        open={open}
        onClose={handleClose}
        closeAfterTransition
        BackdropComponent={Backdrop}
        BackdropProps={{
          timeout: 500,
        }}
      >
        <Fade in={open}>
          <Card sx={style}>
            <CardHeader
              avatar={
                <Avatar sx={{ bgcolor: red[500] }} aria-label="recipe">
                  R
                </Avatar>
              }
              action={
                <IconButton aria-label="close" onClick={handleClose}>
                  <CloseIcon />
                </IconButton>
              }
              title="Shrimp and Chorizo Paella"
              subheader="September 14, 2016"
            />
            <CardMedia>
              <Box sx={{ width: "100%" }}>
                <Box>
                  <AntTabs
                    value={value}
                    onChange={handleChange}
                    textColor="secondary"
                    indicatorColor="secondary"
                    aria-label="secondary tabs example"
                  >
                    <AntTab label="Item One" {...a11yProps(0)} />
                    <AntTab label="Item Two" {...a11yProps(1)} />
                    <AntTab label="Item Three" {...a11yProps(2)} />
                  </AntTabs>
                </Box>
              </Box>
            </CardMedia>
            <CardContent sx={{ height: 300 }}>
              <TabPanel value={value} index={0}>
                Item One
              </TabPanel>
              <TabPanel value={value} index={1}>
                Item Two
                <Typography gutterBottom variant="h5" component="div">
                  Lizard
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  Lizards are a widespread group of squamate reptiles, with over
                  6,000 species, ranging across all continents except Antarctica
                </Typography>
              </TabPanel>
              <TabPanel value={value} index={2}>
                Item Three
              </TabPanel>
            </CardContent>
            <CardActions sx={{ borderTop: "1px solid #e8e8e8" }}>
              <Grid
                container
                direction="row"
                justifyContent="flex-end"
                alignItems="center"
              >
                <Button
                  variant="outlined"
                  size="medium"
                  sx={{ m: 1 }}
                  onClick={handleClose}
                >
                  Cancel
                </Button>
                <Button variant="contained" size="medium">
                  Save
                </Button>
              </Grid>
            </CardActions>
          </Card>
        </Fade>
      </Modal>
    </div>
  );
}
