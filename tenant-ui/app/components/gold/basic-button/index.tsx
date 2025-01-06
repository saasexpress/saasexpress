import { Link, Button } from "@mui/material";
import type { ButtonProps, ButtonOwnProps } from "@mui/material";
// import PlusIcon from "@mui/icons-material/Add";

//import '../../../lib/variants.d.ts';

// type CustomVariant = 'condensed';

// Extend the ButtonProps type with your custom variant
// interface CustomButtonProps extends Omit<ButtonProps, 'variant'> {
//   variant?: CustomVariant;
// }

// declare module '@mui/material/Button' {
//   interface ButtonPropsVariantOverrides {
//     condensed: true;
//   }
// }

// export interface ButtonPropsVariantOverrides {
//   condensed: true;
// }

interface BasicButtonProps extends ButtonProps {
  key?: string;
  href?: string;
  onClick?: any;
  children: any;
  icon?: any;
  disabled?: boolean;
  color?: "primary" | "secondary";
  type?: "submit" | "button" | "reset";
  variant?: any;
}

export default function BasicButton({
  key,
  href,
  onClick,
  children,
  icon,
  disabled,
  type,
  color = "primary",
  variant = "contained",
}: BasicButtonProps) {
  if (href) {
    return (
      <Link key={key} href={href} onClick={onClick}>
        <Button variant={variant} size="large" color={color} startIcon={icon}>
          {children}
        </Button>
      </Link>
    );
  } else {
    return (
      <Button
        key={key}
        type={type}
        onClick={onClick}
        variant={variant}
        disabled={disabled}
        size="large"
        color={color}
        startIcon={icon}
      >
        {children}
      </Button>
    );
  }
}
