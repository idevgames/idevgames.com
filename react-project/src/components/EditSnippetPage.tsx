import { useEffect, useMemo, useState } from 'react';
import { useParams } from 'react-router-dom';
import { HttpClient } from '../client/client';
import { Snippet } from '../client/snippets';
import { useAppSelector } from '../hooks';
import SnippetForm from './SnippetForm';
import AdminOnly from './AdminOnly';

export default function EditSnippetPage() {
  return <AdminOnly>
    <LoadSnippet />
  </AdminOnly>;
}

type RouteParams = {
  taxonomy: string,
  snippetId: string,
};

function LoadSnippet() {
  const snippetId = parseInt(useParams<RouteParams>().snippetId);
  const clientProps = useAppSelector(state => state.clientProps);
  const client = useMemo(() => new HttpClient(clientProps), [clientProps]);
  const [snippet, setSnippet] = useState<Snippet | null>(null);

  useEffect(() => {
    client.getSnippet({ snippetId }).then((getSnippetOutput) => {
      setSnippet(getSnippetOutput.snippet);
    });
  }, [client, snippetId]);

  if (snippet) {
    return <SnippetForm title="Edit snippet" snippet={snippet} onSubmit={(values => {
      client.updateSnippet({ ...values, id: snippetId, taxonomy: snippet.taxonomy, })
        .then((_updateSnippetOutput) => {
          setSnippet({ ...snippet, ...values, });
        });
    })} />
  } else {
    return <>Loading...</>;
  }
}
