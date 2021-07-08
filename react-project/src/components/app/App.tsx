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

export interface AppProps { }

export default function App(_props: AppProps) {
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
          <Route path="/snippets/:taxonomy">
            About
          </Route>
          <Route path="/users">
            Bar
          </Route>
        </Switch>
      </div>
      <Footer />
    </Router>
  );
}
