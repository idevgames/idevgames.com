
/**
 * A snippet, which is one of those little tidbits of information that
 * we leave on the site to make it look like people still live here.
 */
export interface Snippet {
  /**
   * The database id of this snippet.
   */
  id: number;
  /**
   * The id of the user who created this snippet.
   */
  creatorId: number;
  /**
   * The taxonomy of this snippet, which is roughly the collection the
   * snippet belongs to.
   */
  taxonomy: string;
  /**
   * Whether the snippet is hidden from view or not.
   */
  hidden: boolean;
  /**
   * The title of this snippet.
   */
  title: string;
  /**
   * The icon associated with this snippet.
   */
  icon: string;
  /**
   * Who shared this snippet.
   */
  sharedBy: string;
  /**
   * When this snippet was shared. I try to make sure this is the date
   * it was shared on Discord, not the date I actually bothered to put
   * it on the site.
   */
  sharedOn: string;
  /**
   * A summary of this snippet.
   */
  summary: string;
  /**
   * A longform description of this snippet. Currently unused and most
   * snippets don't have this filled in.
   */
  description: string;
  /**
   * Snippets are links to further content. This is that link.
   */
  href: string;
  /**
   * When this snippet was created.
   */
  createdAt: string;
  /**
   * The date of the last update to this snippet.
   */
  updatedAt: string;
}

/**
 * Information required to create a Snippet. Items such as id,
 * creatorId, createdAt, and updatedAt are inferred by the server and
 * cannot be set manually.
 */
export interface CreateSnippetInput {
  /**
   * The taxonomy of this snippet, which is roughly the collection the
   * snippet belongs to.
   */
  taxonomy: string;
  /**
   * Whether the snippet is hidden from view or not.
   */
  hidden: boolean;
  /**
   * The title of this snippet.
   */
  title: string;
  /**
   * The icon associated with this snippet.
   */
  icon: string;
  /**
   * Who shared this snippet.
   */
  sharedBy: string;
  /**
   * When this snippet was shared. I try to make sure this is the date
   * it was shared on Discord, not the date I actually bothered to put
   * it on the site.
   */
  sharedOn: string;
  /**
   * A summary of this snippet.
   */
  summary: string;
  /**
   * A longform description of this snippet. Currently unused and most
   * snippets don't have this filled in.
   */
  description: string;
  /**
   * Snippets are links to further content. This is that link.
   */
  href: string;
}

/**
 * The response from creating a snippet.
 */
export interface CreateSnippetOutput {
  /**
   * The Snippet that was created.
   */
  snippet: Snippet;
}

/**
 * Input for listing snippets.
 */
export interface ListSnippetInput {
  /**
   * The taxonomy of snippets to look in.
   */
  taxonomy: string;
  /**
   * The page number to fetch; defaults to zero.
   */
  page?: number;
  /**
   * Whether to show hidden snippets. You must be logged in and marked
   * as an admin for this to work. Defaults to false.
   */
  showHidden?: boolean;
}

/**
 * Output from the list snippets call.
 */
export interface ListSnippetOutput {
  /**
   * The snippets that were retrieved.
   */
  snippets: Array<Snippet>;
  /**
   * The current page in the list of snippets.
   */
  currentPage: number;
  /**
   * The total number of pages in the snippet list.
   */
  totalPages: number;
}
