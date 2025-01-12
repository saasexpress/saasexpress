import { styled } from "@mui/material/styles";
import Tooltip, { tooltipClasses } from "@mui/material/Tooltip";

const BootstrapTooltip = styled(({ className, ...props }: any) => (
  <Tooltip {...props} arrow classes={{ popper: className }} />
))(({ theme }) => ({
  [`& .${tooltipClasses.arrow}`]: {
    color: theme.palette.common.black,
  },
  [`& .${tooltipClasses.tooltip}`]: {
    backgroundColor: theme.palette.common.black,
    fontSize: "1rem",
  },
}));

export default function CustomizedTooltip(
  props = {
    placement: "bottom",
    tooltip: "" as any,
    children: undefined as any,
  }
) {
  const { tooltip, children, placement } = props;

  return (
    <BootstrapTooltip title={tooltip} placement={placement}>
      {children}
    </BootstrapTooltip>
  );
}
