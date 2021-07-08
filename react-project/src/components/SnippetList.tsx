import { Fragment, useState } from 'react';
import ShortSnippet from './shortsnippet/ShortSnippet';
import { Snippet } from '../client/snippets';
import { useAppSelector } from '../hooks';
import { Link } from 'react-router-dom';
import { HttpClient } from '../client/client';
import { useEffect } from 'react';
import { useMemo } from 'react';

export interface SnippetListProps {
  key: string,
  taxonomy: string,
  page: number,
  /**
   * Show a pager, otherwise it shows a link to "More."
   */
  showPager: boolean,
}

export interface SnippetListState {
  taxonomy: string,
  page: number,
  totalPages: number,
  snippets: Snippet[],
};

export default function SnippetList(props: SnippetListProps) {
  const clientProps = useAppSelector(state => state.clientProps);
  const client = useMemo(() => new HttpClient(clientProps), [clientProps]);
  const [page, setPage] = useState<number>(props.page);
  const [state, setState] = useState<SnippetListState | null>(null);

  // load snippets, async and only once
  useEffect(() => {
    client.listSnippets({
      taxonomy: props.taxonomy, page: page,
    }).then(listSnippetsOutput => {
      setState({
        taxonomy: props.taxonomy,
        page: listSnippetsOutput.currentPage,
        totalPages: listSnippetsOutput.totalPages,
        snippets: listSnippetsOutput.snippets,
      });
      setPage(listSnippetsOutput.currentPage);
    });
  }, [client, props.taxonomy, page]);

  if (state === null) {
    return <div className="snippet-list">Loading...</div>;
  } else {
    return <div className="snippet-list">
      {state.snippets.map((snippet) => {
        return <ShortSnippet key={snippet.id} snippet={snippet} />;
      })}
      {props.showPager && <Fragment>Pager here</Fragment>}
      {!props.showPager && <MoreLink taxonomy={state.taxonomy} />}
    </div>;
  }
}

interface MoreLinkProps {
  taxonomy: string,
}

function MoreLink(props: MoreLinkProps) {
  const session = useAppSelector(state => state.session);
  return <p>
    {session.permissions.includes('admin') &&
      <Fragment>
        <Link to={`/snippets/${props.taxonomy}/new`}>New</Link>&nbsp;&middot;&nbsp;
      </Fragment>}
    <Link to={`/snippets/${props.taxonomy}?page=1`}>More &rarr;</Link>
  </p>;
}
