import { Fragment, useEffect, useMemo, useState } from 'react';
import { useParams } from 'react-router-dom';
import { useAppSelector } from '../hooks';
import { HttpClient } from '../client/client';
import { Snippet } from '../client/snippets';
import ShortSnippet from './shortsnippet/ShortSnippet';

type RouteParams = {
  taxonomy: string,
  snippetId: string,
};

export default function SingleSnippet() {
  const snippetId = parseInt(useParams<RouteParams>().snippetId);
  const clientProps = useAppSelector(state => state.clientProps);
  const client = useMemo(() => new HttpClient(clientProps), [clientProps]);
  const [snippet, setSnippet] = useState<Snippet | null>(null);

  useEffect(() => {
    client.getSnippet({ snippetId }).then((getSnippetOutput) => {
      setSnippet(getSnippetOutput.snippet);
    });
  });

  if (snippet) {
    return <ShortSnippet key={snippetId} snippet={snippet} />;
  } else {
    return <Fragment />;
  }
}