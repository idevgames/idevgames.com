import React from 'react';
import ShortSnippet from '../shortsnippet/ShortSnippet';
import { Snippet } from '../../client/snippets';
import { HttpClient } from '../../client/client';

export default class SnippetList extends React.Component<SnippetListProps, SnippetListState> {
  constructor(props: SnippetListProps) {
    super(props);
    const client = new HttpClient();
    const snippetListOutput = client.listSnippets({
      taxonomy: props.taxonomy, page: props.page
    });
    this.state = {
      taxonomy: props.taxonomy,
      page: props.page,
      totalPages: snippetListOutput.totalPages,
      snippets: snippetListOutput.snippets,
    };
  }
  render() {
    const snippetList = this.state.snippets.map((snippet) => (
      <ShortSnippet key={snippet.id} snippet={snippet}/>
    ));
    return (
      <div className="snippet-list">
        { snippetList }
      </div>
    )
  }
}

export interface SnippetListProps {
  key: string;
  taxonomy: string;
  page: number;
}

export interface SnippetListState {
  taxonomy: string;
  page: number;
  totalPages: number;
  snippets: Snippet[];
}
