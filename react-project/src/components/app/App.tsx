import {
  BrowserRouter as Router,
  Switch,
  Route,
} from "react-router-dom";
import './App.scss';
import Header from '../header/Header';
import Footer from '../Footer';
import Homepage from '../homepage/Homepage';
import GithubCallback from '../GithubCallback';
import SnippetsPage from '../SnippetsPage';
import SingleSnippet from '../SingleSnippet';
import EditSnippetPage from '../EditSnippetPage';

export default function App() {
  return (
    <Router>
      <Header />
      <div className="superbox">
        <Switch>
          <Route exact path="/">
            <Homepage />
          </Route>
          <Route path="/github_callback">
            <GithubCallback />
          </Route>
          <Route path="/snippets/:taxonomy/page/:page">
            <SnippetsPage />
          </Route>
          <Route path="/snippets/:taxonomy/:snippetId/edit">
            <EditSnippetPage />
          </Route>
          <Route path="/snippets/:taxonomy/:snippetId">
            <SingleSnippet />
          </Route>
        </Switch>
      </div>
      <Footer />
    </Router>
  );
}
