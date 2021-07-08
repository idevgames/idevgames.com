import { Fragment, useState } from 'react';
import ShortSnippet from './shortsnippet/ShortSnippet';
import { Snippet } from '../client/snippets';
import { useAppSelector } from '../hooks';
import { Link } from 'react-router-dom';
import { HttpClient } from '../client/client';
import { useEffect } from 'react';
import { useMemo } from 'react';
import { snippetsPage } from '../namedRoutes';

export interface SnippetListProps {
  taxonomy: string,
  page: number,
  /**
   * Show a pager, otherwise it shows a link to "More."
   */
  showPager: boolean,
  showHidden: boolean,
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
  const [state, setState] = useState<SnippetListState | null>(null);

  // load snippets, async and only once
  useEffect(() => {
    client.listSnippets({
      taxonomy: props.taxonomy, page: props.page, showHidden: props.showHidden
    }).then(listSnippetsOutput => {
      setState({
        taxonomy: props.taxonomy,
        page: listSnippetsOutput.currentPage,
        totalPages: listSnippetsOutput.totalPages,
        snippets: listSnippetsOutput.snippets,
      });
    });
  }, [client, props.taxonomy, props.showHidden, props.page]);

  if (state === null) {
    return <div className="snippet-list">Loading...</div>;
  } else {
    return <div className="snippet-list">
      {state.snippets.map((snippet) => {
        return <ShortSnippet key={snippet.id} snippet={snippet} />;
      })}
      {props.showPager && <Pager taxonomy={state.taxonomy} currentPage={state.page} totalPages={state.totalPages} showHidden={props.showHidden} />}
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
    <Link to={snippetsPage(props.taxonomy, 1, false)}>More &rarr;</Link>
  </p>;
}

interface PagerProps {
  taxonomy: string,
  currentPage: number,
  totalPages: number,
  showHidden: boolean,
}

function Pager(props: PagerProps) {
  return <nav>
    <ul className="pagination justify-content-center">
      <li key="previous" className={previousPageClasses(props.currentPage)}>
        <Link className="page-link" to={snippetsPage(props.taxonomy, Math.max(0, props.currentPage - 1), props.showHidden)}>
          Previous
        </Link>
      </li>
      {pagerRange(props.currentPage, props.totalPages - 1).map(page => <li key={page} className={pageClasses(page, props.currentPage)}>
        <Link className="page-link" to={snippetsPage(props.taxonomy, page, props.showHidden)}>{page + 1}</Link>
      </li>)}
      <li key="next" className={nextPageClasses(props.currentPage, props.totalPages)}>
        <Link className="page-link" to={snippetsPage(props.taxonomy, Math.min(props.totalPages - 1, props.currentPage + 1), props.showHidden)}>
          Next
        </Link>
      </li>
    </ul>
  </nav>;
}

function previousPageClasses(currentPage: number): string {
  let classes = ['page-item'];
  if (currentPage === 0) {
    classes.push('disabled');
  }
  return classes.join(' ');
}

function pageClasses(itemPage: number, currentPage: number): string {
  let classes = ['page-item'];
  if (currentPage === itemPage) {
    classes.push('disabled');
  }
  return classes.join(' ');
}

function nextPageClasses(currentPage: number, totalPages: number): string {
  let classes = ['page-item'];
  if (currentPage === totalPages) {
    classes.push('disabled');
  }
  return classes.join(' ');
}

function pagerRange(currentPage: number, totalPages: number): number[] {
  const pagerWidth = 5;
  const lowerBound = Math.max(0, currentPage - pagerWidth);
  const upperBound = Math.min(totalPages, currentPage + pagerWidth);
  const pages = [];
  for (let i = lowerBound; i <= upperBound; i = i + 1) {
    pages.push(i);
  }
  return pages;
}
