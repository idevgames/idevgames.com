import React from 'react';
import { Fragment } from 'react';
import { setSession } from '../session';
import { useAppDispatch, useAppSelector } from '../hooks';
import { GetGithubAuthorizationUrlOutput } from '../client/auth';
import { useLocation } from 'react-router-dom';
import { useEffect } from 'react';
import { HttpClient } from '../client/client';

export default function SessionButton(_props: any) {
  const client = new HttpClient(useAppSelector(state => state.clientProps));
  const session = useAppSelector(state => state.session);
  const dispatch = useAppDispatch();
  const location = useLocation();

  const doLogin = (e: React.MouseEvent) => {
    e.preventDefault();
    const authUrl = client.getGithubAuthorizationUrl();
    authUrl.then((getGithubAuthorizationUrlOutput: GetGithubAuthorizationUrlOutput) => {
      window.location.href = getGithubAuthorizationUrlOutput.url;
    });
  };

  const doLogout = (e: React.MouseEvent) => {
    e.preventDefault();
    let sessionDeleted = client.deleteSession();
    sessionDeleted.then((_: any) => {
      // clear the session identity and permissions in context
      dispatch(
        setSession({ sessionIdentity: null, permissions: [] })
      );
    });
  };

  // this part detects any current session and sets the client-side
  // session; on the github callback page we have a component that's
  // logging us in, so we shouldn't try to query the login because that
  // might set the client-side session information in a race condition.
  useEffect(() => {
    if (!location.pathname.startsWith("/github_callback")) {
      client.getSession()
        .then(output => {
          dispatch(setSession({ sessionIdentity: output.user, permissions: output.permissions }));
        })
        .catch(oops => {
          console.log('Failed to get session', oops);
          dispatch(setSession({ sessionIdentity: null, permissions: [] }));
        });
    }
    // disable on purpose, the lack of dependencies ensures this only
    // runs once
    // eslint-disable-next-line
  }, []);

  if (session.sessionIdentity === null) {
    return <Fragment>
      Editors of the site can <a href="#login" onClick={doLogin}>login</a>.
    </Fragment>;
  } else {
    return <Fragment>
      Hello {session.sessionIdentity?.login}!&nbsp;
      <a href="#logout" onClick={doLogout}>Logout</a>
    </Fragment>;
  }
}
