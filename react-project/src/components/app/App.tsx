import React from 'react';
import {
  BrowserRouter as Router,
  Switch,
  Route,
} from "react-router-dom";
import './App.scss';
import Header from '../header/header';
import Footer from '../footer/footer';
import Homepage from '../homepage/Homepage';
import ApplicationStateProvider from '../../application_state';
import { ApplicationState } from '../../application_state'

export default class App extends React.Component {
  componentDidMount() {
  }
  render() {
    return (
      <ApplicationStateProvider>
        <Router>
          <Header/>
          <div className="superbox">
            <Switch>
              <Route path="/">
                <Homepage/>
              </Route>
              <Route path="/about">
                About
              </Route>
              <Route path="/users">
                Bar
              </Route>
            </Switch>
          </div>

          <Footer/>
        </Router>
      </ApplicationStateProvider>
    )
  }
}
