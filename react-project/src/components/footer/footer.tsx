import React from 'react';

export default class Footer extends React.Component {
  render() {
    return (
      <footer>
        <div className="row m-2">
          <div className="col-md-3"></div>
          <div className="col-md-6">
            <p>
              &copy; 1998 - {(new Date()).getFullYear()} <a href="https://www.idevgames.com/">iDevGames</a>. All rights reserved.
            </p>
            <p>
              Found a bug? Please report it either on <a href="https://discord.gg/r6fxVBH">Discord</a> or on <a href="https://github.com/idevgames/idevgames.com">Github</a>. Please responsibly disclose security issues directly to <abbr title="Necrothitude#0292">@mysteriouspants</abbr> over Discord Private Messaging.
            </p>
          </div>
          <div className="col-md-3"></div>
        </div>
      </footer>
    )
  }
}
