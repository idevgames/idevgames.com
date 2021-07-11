import { useMemo } from 'react';
import { useHistory, useParams } from 'react-router-dom';
import { HttpClient } from '../client/client';
import { Snippet } from '../client/snippets';
import { useAppSelector } from '../hooks';
import AdminOnly from './AdminOnly';
import SnippetForm from './SnippetForm';
import icons from '../icons';
import { editSnippetPage } from '../namedRoutes';

interface RouteParams {
  taxonomy: string,
}

export default function CreateSnippetPage() {
  const history = useHistory();
  const route = useParams<RouteParams>();
  const taxonomy = route.taxonomy;
  const clientProps = useAppSelector(state => state.clientProps);
  const client = useMemo(() => new HttpClient(clientProps), [clientProps]);

  const blankSnippet: Snippet = {
    id: -1,
    creatorId: -1,
    taxonomy,
    hidden: true,
    title: '',
    icon: icons['Safari'],
    sharedBy: '',
    sharedOn: new Date(Date.now()),
    summary: '',
    description: '',
    href: '',
    createdAt: new Date(Date.now()),
    updatedAt: new Date(Date.now()),
  };

  return <AdminOnly>
    <SnippetForm title="New snippet" snippet={blankSnippet} onSubmit={(values => {
      client.createSnippet({ ...values, taxonomy, })
        .then((createSnippetOutput) => {
          history.push(editSnippetPage(taxonomy, createSnippetOutput.snippet.id));
        });
    })} />
  </AdminOnly>;
}
