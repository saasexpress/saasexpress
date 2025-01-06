import * as React from "react";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogTitle from "@mui/material/DialogTitle";
import BasicButton from "@components/gold/basic-button";
import NewButton from "../new-button";
import { Box, Step, StepLabel, Stepper, Typography } from "@mui/material";
// import CustomizedTooltip from '../tooltip-simple';
// import MouseOverPopover from '../mouse-over-popover';

interface WizardStep {
  optional: boolean;
  label: string;
}

interface WizardDialogProps {
  children: React.ReactNode[];
  buttonLabel: string;
  title: React.ReactNode;
  steps: WizardStep[];
  onFinish: Function;
}

export default function WizardDialog({
  children,
  buttonLabel,
  title,
  steps,
  onFinish,
}: WizardDialogProps) {
  const [open, setOpen] = React.useState(false);
  const [activeStep, setActiveStep] = React.useState(0);
  const [skipped, setSkipped] = React.useState(new Set());

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  const isStepOptional = (step: number) => {
    return steps[step].optional;
  };

  const isStepSkipped = (step: number) => {
    return skipped.has(step);
  };

  const handleNext = () => {
    let newSkipped = skipped;
    if (isStepSkipped(activeStep)) {
      newSkipped = new Set(newSkipped.values());
      newSkipped.delete(activeStep);
    }

    setActiveStep((prevActiveStep) => prevActiveStep + 1);
    setSkipped(newSkipped);
  };

  const handleBack = () => {
    setActiveStep((prevActiveStep) => prevActiveStep - 1);
  };

  const handleSkip = () => {
    if (!isStepOptional(activeStep)) {
      // You probably want to guard against something like this,
      // it should never occur unless someone's actively trying to break something.
      throw new Error("You can't skip a step that isn't optional.");
    }

    setActiveStep((prevActiveStep) => prevActiveStep + 1);
    setSkipped((prevSkipped) => {
      const newSkipped = new Set(prevSkipped.values());
      newSkipped.add(activeStep);
      return newSkipped;
    });
  };

  const handleReset = () => {
    setActiveStep(0);
  };

  // React.useEffect(() => {
  //   if (activeStep === steps.length) {
  //     onFinish();
  //   }
  // }, [activeStep]);

  return (
    <React.Fragment>
      <NewButton onClick={handleClickOpen}>{buttonLabel}</NewButton>
      <Dialog
        fullWidth={true}
        maxWidth="md"
        scroll="paper"
        open={open}
        onClose={handleClose}
        PaperProps={{
          component: "form",
          onSubmit: (event: React.SyntheticEvent) => {
            event.preventDefault();
            const formData = new FormData(
              event.currentTarget as HTMLFormElement
            );
            const formJson = Object.fromEntries(formData.entries());
            console.log(JSON.stringify(formJson));
            //handleClose();
            onFinish(formJson);
          },
        }}
      >
        <DialogTitle mb={0} sx={{ paddingBottom: 0 }}>
          {title}
          <Stepper activeStep={activeStep} alternativeLabel>
            {steps.map((step, index) => {
              const stepProps = {};
              const labelProps: { optional?: React.ReactNode } = {};
              if (step.optional) {
                labelProps.optional = (
                  <Typography variant="caption">Optional</Typography>
                );
              }
              // if (isStepSkipped(index)) {
              //   stepProps.completed = false;
              // }
              return (
                <Step key={step.label} {...stepProps}>
                  <StepLabel {...labelProps}>{step.label}</StepLabel>
                </Step>
              );
            })}
          </Stepper>
        </DialogTitle>
        <DialogContent sx={{ height: "400px" }}>
          {steps
            .filter((step, index) => activeStep === index)
            .map((step, index) => {
              return (
                <Box
                  sx={{
                    // marginTop: 2,
                    paddingLeft: 5,
                    paddingRight: 5,
                    // backgroundColor: '#EFEFEF',
                    borderRadius: 0,
                    // minHeight: '200px',
                    // transition: 'height 200ms',
                  }}
                >
                  {children[activeStep]}
                </Box>
              );
            })}
        </DialogContent>
        <DialogActions>
          <BasicButton
            variant="text"
            disabled={activeStep === 0}
            onClick={handleBack}
            sx={{ mr: 1 }}
          >
            Back
          </BasicButton>
          <Box sx={{ flex: "1 1 auto" }} />
          {steps[activeStep].optional && (
            <BasicButton variant="outlined" onClick={handleSkip} sx={{ mr: 1 }}>
              Skip
            </BasicButton>
          )}
          {activeStep === steps.length - 1 ? (
            <BasicButton key="btn-submit" type="submit">
              Submit
            </BasicButton>
          ) : (
            <BasicButton key="btn-next" onClick={handleNext}>
              Next
            </BasicButton>
          )}
          {/* <BasicButton onClick={handleClose}>Cancel</BasicButton>
          <BasicButton type="submit">Submit</BasicButton> */}
        </DialogActions>
      </Dialog>
    </React.Fragment>
  );
}
