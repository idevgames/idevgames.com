import React from 'react';
import { Client, HttpClient } from './client/client';
import { SessionIdentity } from './client/auth';

export interface IApplicationState {
  readonly client: Client;
  readonly sessionIdentity: SessionIdentity | null;
  readonly permissions: string[];
}

export let defaultState = function (): IApplicationState {
  return {
    client: new HttpClient(),
    sessionIdentity: null,
    permissions: []
  };
}

export let ApplicationState = React.createContext<IApplicationState>(defaultState());

export default class ApplicationStateProvider extends React.Component {
  readonly state: IApplicationState;
  constructor(props: any) {
    super(props);
    this.state = defaultState();
  }
  render() {
    return (
      <ApplicationState.Provider value={this.state}>
        {this.props.children}
      </ApplicationState.Provider>
    );
  }
}
