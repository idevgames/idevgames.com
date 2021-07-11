export function snippetsPage(taxonomy: string, page: number, showHidden: boolean): string {
    let route = `/snippets/${taxonomy}/page/${page}`;
    if (showHidden) {
        route += '?showHidden=true';
    }
    return route;
}

export function snippetPage(taxonomy: string, snippetId: number): string {
    return `/snippets/${taxonomy}/${snippetId}`;
}

export function editSnippetPage(taxonomy: string, snippetId: number): string {
    return snippetPage(taxonomy, snippetId) + '/edit';
}
