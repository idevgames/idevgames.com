import React from 'react';
import { Link } from 'react-router-dom';
import header_small from './header_small.png';

export default class Header extends React.Component {
  render() {
    return (
      <header className="container">
        <Link to="/">
          <img src={header_small} alt="Cmd-R is watching you!"/>
        </Link>
      </header>
    )
  }
}
