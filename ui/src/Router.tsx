import { HashRouter, Switch, Route } from "react-router-dom";
import { IncludeApollo } from "./apollo";
import AudienceDisplayPage from "./pages/AudienceDisplayPage";
import LinksPage from "./pages/LinksPage";
import LoginPage from "./pages/LoginPage";
import MatchTestPage from "./pages/MatchTestPage";
import RefPanelPage from "./pages/RefPanelPage";
import SoundsPage from "./pages/SoundsPage";

export default function Router() {
  return (
    <HashRouter>
      <IncludeApollo>
        <Switch>

        <Route path="/login" exact>
            <LoginPage />
          </Route>

          <Route path="/control" exact>
            <MatchTestPage />
          </Route>

          <Route path="/sounds" exact>
            <SoundsPage />
          </Route>

          <Route path="/audience" exact>
            <AudienceDisplayPage />
          </Route>

          <Route path="/refpanel" exact>
            <RefPanelPage />
          </Route>

          <Route>
            <LinksPage />
          </Route>
          
        </Switch>
      </IncludeApollo>
    </HashRouter>
  )
}