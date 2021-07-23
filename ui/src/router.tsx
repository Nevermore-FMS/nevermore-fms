import { Container, Typography } from "@material-ui/core";
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";
import NavBar from "./components/NavBar";

export default function NevermoreRouter() {
  return (
    <Router>
      <NavBar />

      <Container>
        <Switch>
          <Route exact path="/">
            <Home />
          </Route>
          <Route path="/about">
            <About />
          </Route>
          <Route path="/dashboard">
            <Dashboard />
          </Route>
        </Switch>
      </Container>
    </Router>
  );
}

// You can think of these components as "pages"
// in your app.

function Home() {
  return (
    <div>
      <Typography>Home</Typography>
    </div>
  );
}

function About() {
  return (
    <div>
      <Typography>About</Typography>
    </div>
  );
}

function Dashboard() {
  return (
    <div>
      <Typography>Dashboard</Typography>
    </div>
  );
}
