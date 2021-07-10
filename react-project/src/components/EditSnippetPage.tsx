import { useEffect, useMemo, useState } from 'react';
import { useParams } from 'react-router-dom';
import { HttpClient } from '../client/client';
import { Snippet } from '../client/snippets';
import { useAppSelector } from '../hooks';
import SnippetForm from './SnippetForm';

export default function EditSnippetPage() {
  return <EditSnippetAuthorization />;
}

function EditSnippetAuthorization() {
  const session = useAppSelector(state => state.session);

  if (!session.permissions.includes('admin')) {
    // this is something of a formality, the backend will reject edits
    // anyway.
    return <>
      <h1>Not authorized</h1>
      <p>
        Only editors may modify this site content.
      </p>
    </>;
  } else {
    return <LoadSnippet />;
  }
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
    return <SnippetForm snippet={snippet} onSubmit={(values => {
      client.updateSnippet({ ...values, id: snippetId, taxonomy: snippet.taxonomy, })
        .then((_updateSnippetOutput) => {
          setSnippet({ ...snippet, ...values, });
        });
    })} />
  } else {
    return <>Loading...</>;
  }
}
