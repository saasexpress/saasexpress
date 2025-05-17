import {
  Link,
  ListItemButton,
  IconButton,
  Menu,
  MenuItem,
  Avatar,
} from "@mui/material";
import { AccountCircle, Login } from '@mui/icons-material';
import useAPIClient from "lib/api/APIClient";
import { useState, MouseEvent } from "react";

interface SigninProps {}

let Signin = (props: SigninProps) => {
  let api = useAPIClient();
  let {isPending, data} = api.get(["me"], "/auth/session");
  
  // Menu state
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);
  
  const handleClick = (event: MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };
  
  const handleClose = () => {
    setAnchorEl(null);
  };


  if (isPending) {
    return <></>;
  }

  return data?.data?.sub ? (
    <div>
      <IconButton
        onClick={handleClick}
        size="large"
        aria-controls={open ? 'account-menu' : undefined}
        aria-haspopup="true"
        aria-expanded={open ? 'true' : undefined}
      >
        <Avatar sx={{ width: 32, height: 32 }}>
          <AccountCircle />
        </Avatar>
      </IconButton>
      <Menu
        id="account-menu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        transformOrigin={{ horizontal: 'right', vertical: 'top' }}
        anchorOrigin={{ horizontal: 'right', vertical: 'bottom' }}
      >
        <MenuItem disabled>
          {data.data.sub}
        </MenuItem>
        <MenuItem component={Link} href="/auth/signout" onClick={handleClose}>
          Logout
        </MenuItem>
      </Menu>
    </div>
  ) : (
    <Link href={"/auth/signin"} style={{ textDecoration: 'none' }}>
      <ListItemButton sx={{ mr: 1, p: 1 }}>
        <Login sx={{ mr: 1 }} />
        LOGIN
      </ListItemButton>
    </Link>
  );
}

export default Signin;