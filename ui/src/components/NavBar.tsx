import {
  IconButton,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  SwipeableDrawer,
  Toolbar,
  Typography,
} from "@material-ui/core";
import AppBar from "@material-ui/core/AppBar";
import MenuIcon from "@material-ui/icons/Menu";
import { useState, ReactNode } from "react";
import { Link } from "react-router-dom";

interface NavLink {
  to: string;
  primary: string;
  secondary: string | null;
  icon: ReactNode;
}

let links: NavLink[] = [
  {
    to: "/test",
    primary: "Test",
    secondary: null,
    icon: <MenuIcon />,
  },
];

export default function NavBar() {
  let [drawerOpen, setDrawerOpen] = useState(false);

  return (
    <div>
      <AppBar position="static">
        <Toolbar>
          <IconButton
            edge="start"
            color="inherit"
            aria-label="menu"
            onClick={() => setDrawerOpen(true)}
          >
            <MenuIcon></MenuIcon>
          </IconButton>
          <Typography variant="h6">Nevermore</Typography>
        </Toolbar>
      </AppBar>
      <SwipeableDrawer
        open={drawerOpen}
        onClose={() => setDrawerOpen(false)}
        onOpen={() => setDrawerOpen(true)}
      >
        <List>
          {links.map((link) => (
            <ListItem key={link.to} component={Link} to={link.to}>
              <ListItemIcon>{link.icon}</ListItemIcon>
              <ListItemText
                primary={link.primary}
                secondary={link.secondary}
              ></ListItemText>
            </ListItem>
          ))}
        </List>
      </SwipeableDrawer>
    </div>
  );
}
