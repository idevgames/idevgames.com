import React from 'react';
import { ApplicationState } from '../application_state';
import { GetGithubAuthorizationUrlOutput } from '../client/auth';

export default class SessionButton extends React.Component {
  static contextType = ApplicationState;

  constructor(props: any) {
    super(props);

    this.doLogin = this.doLogin.bind(this);
    this.doLogout = this.doLogout.bind(this);
  }

  render () {
    if (this.context.sessionIdentity === null) {
      return this.renderLogin();
    } else {
      return this.renderLogout();
    }
  }

  renderLogin() {
    return <a href="/foo" onClick={this.doLogin}>Login</a>;
  }

  renderLogout() {
    return <a href="/foo" onClick={this.doLogout}>Logout</a>;
  }

  doLogin(e: React.MouseEvent) {
    e.preventDefault();
    const authUrl = this.context.client.getGithubAuthorizationUrl({});
    authUrl.then((getGithubAuthorizationUrlOutput: GetGithubAuthorizationUrlOutput) => {
      window.location.href = getGithubAuthorizationUrlOutput.url;
    });
  }

  doLogout(e: React.MouseEvent) {
    e.preventDefault();
    let sessionDeleted = this.context.client.deleteSession({});
    sessionDeleted.then((_: any) => {
      // clear the session identity and permissions in context
    });
  }
}