import { Link, Button } from '@mui/material';
import PlusIcon from '@mui/icons-material/Add';

interface NewButtonProps {
  href?: string;
  onClick?: any;
  children: any;
}

export default function NewButton({ href, onClick, children }: NewButtonProps) {
  if (href) {
    return (
      <Link href={href} onClick={onClick}>
        <Button
          variant="contained"
          size="large"
          color="primary"
          startIcon={<PlusIcon />}
        >
          {children}
        </Button>
      </Link>
    );
  } else {
    return (
      <Button
        onClick={onClick}
        variant="contained"
        size="large"
        color="primary"
        startIcon={<PlusIcon />}
      >
        {children}
      </Button>
    );
  }
}
