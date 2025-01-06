import { Typography, TypographyOwnProps } from "@mui/material";

interface BlockDescriptionProps {
  variant: TypographyOwnProps["variant"];
  description: string;
}

export default function BlockDescription({
  variant,
  description,
}: BlockDescriptionProps) {
  return (
    <Typography
      variant={variant}
      sx={{
        display: "-webkit-box",
        boxOrient: "vertical",
        lineClamp: 2,
        wordBreak: "break-word",
        overflow: "hidden",
      }}
    >
      {description}
    </Typography>
  );
}
