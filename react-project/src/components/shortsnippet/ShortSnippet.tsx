import { Link } from 'react-router-dom';
import { Snippet } from '../../client/snippets';
import { useAppSelector } from '../../hooks';
import { snippetPage } from '../../namedRoutes';
import './shortsnippet.scss';

export interface ShortSnippetProps {
  key: number;
  snippet: Snippet;
}

export default function ShortSnippet(props: ShortSnippetProps) {
  const session = useAppSelector(state => state.session);

  // alias this, it just makes life a little easier
  const snippet = props.snippet;

  return (
    <div className="short-snippet">
      <img src={`/icons/${snippet.icon}`} className="icon" alt="" />&nbsp;
      <span className="font-weight-heavy">
        {snippet.sharedBy} shared on {snippet.sharedOn.getFullYear()}-
        {snippet.sharedOn.getMonth().toLocaleString('en', { minimumIntegerDigits: 2 })}-
        {snippet.sharedOn.getDay().toLocaleString('en', { minimumIntegerDigits: 2 })}:&nbsp;
      </span>
      <a href={snippet.href}>{snippet.title}</a>&nbsp;
      {snippet.summary}
      &nbsp;
      {session.permissions.includes("admin") &&
        <span>
          &middot;&nbsp;
          <Link to={`/snippets/${snippet.taxonomy}/${snippet.id}/edit`}>Edit</Link>&nbsp;
        </span>
      }
      <Link to={snippetPage(snippet.taxonomy, snippet.id)} className="text-muted text-decoration-none">#</Link>
    </div>
  );
}
