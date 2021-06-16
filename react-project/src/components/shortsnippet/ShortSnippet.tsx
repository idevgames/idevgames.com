import React from 'react';
import { Link } from 'react-router-dom';
import { Snippet } from '../../client/snippets';
import './shortsnippet.scss';

export default class ShortSnippet extends React.Component<ShortSnippetProps> {
  render() {
    const snippet = this.props.snippet;
    return (
      <span className="short-snippet">
        <img src={`/icons/${snippet.icon}`} className="icon" alt=""/>&nbsp;
        <span className="font-weight-heavy">
          { snippet.sharedBy } shared on { snippet.sharedOn }:&nbsp;
        </span>
        <a href={ snippet.href }>{ snippet.title }</a>&nbsp;
        { snippet.summary }
        &nbsp;&middot;&nbsp;
        <Link to={`/snippets/${snippet.taxonomy}/${snippet.id}/edit`}>Edit</Link>&nbsp;
        <Link to={`/snippets/${snippet.taxonomy}/${snippet.id}`} className="text-muted text-decoration-none">#</Link>
      </span>
    );
  }
}

export interface ShortSnippetProps {
  snippet: Snippet;
}
