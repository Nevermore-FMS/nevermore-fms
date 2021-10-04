import { ApolloProvider } from "@apollo/client";
import { client } from "./apollo";
import Header from "./components/Header";
import MatchTestPage from "./pages/MatchTestPage";

function App() {
  return (
    <div>
        <ApolloProvider client={client}>
            <Header />
            <MatchTestPage />
        </ApolloProvider>
    </div>
  )
}

export default App;
