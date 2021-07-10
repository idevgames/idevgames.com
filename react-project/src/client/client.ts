import { createSlice } from '@reduxjs/toolkit';

import {
  GetSessionOutput, GetGithubAuthorizationUrlOutput, GetGithubCallbackInput,
  GetGithubCallbackOutput, DeleteSessionOutput,
} from './auth';
import {
  CreateSnippetInput, CreateSnippetOutput, GetSnippetInput, GetSnippetOutput,
  ListSnippetInput, ListSnippetOutput, Snippet, UpdateSnippetInput,
  UpdateSnippetOutput,
} from './snippets';

/**
 * Serializable properties used to construct HttpClients.
 */
export interface HttpClientProps {
  /**
   * The base of the url's path part.
   */
  baseUrl: string,
}

/**
 * A client which interacts with the iDevGames server.
 */
export class HttpClient {
  /**
   * The base url path.
   */
  readonly baseUrl: string;

  /**
   * Creates a new HttpClient.
   */
  constructor(props: HttpClientProps) {
    this.baseUrl = props.baseUrl;
  }

  /**
   * Gets the current session identity, if any.
   * @returns the current session identity, if any.
   */
  async getSession(): Promise<GetSessionOutput> {
    const r = await fetch(
      this.baseUrl + '/session',
      this.defaultFetchArgs('GET', null)
    );
    return await r.json();
  }

  /**
   * Gets a Github Authorization URL, which starts the OAuth process.
   * @returns Gets the Github authorization URL, which is where to send
   * the customer to log in with Github.
   */
  async getGithubAuthorizationUrl(): Promise<GetGithubAuthorizationUrlOutput> {
    const response = await fetch(
      this.baseUrl + '/session/github_authorization_url',
      this.defaultFetchArgs('GET', null)
    );
    return response.json();
  }

  /**
   * Takes the OAuth code returned by Github to the user and hands it
   * off to the backend, which then hands it back to Github to establish
   * the chain of trust and log the customer in.
   * @param input Github callback input.
   * @returns The result of the callback, which is a new session
   * identity.
   */
  async getGithubCallback(input: GetGithubCallbackInput): Promise<GetGithubCallbackOutput> {
    const r = await fetch(
      this.baseUrl + `/session/github_callback?code=${input.code}`,
      this.defaultFetchArgs('GET', null)
    );
    return await r.json();
  }

  /**
   * Deletes the current session, logging the user out.
   * @returns Session deletion result.
   */
  async deleteSession(): Promise<DeleteSessionOutput> {
    const response = await fetch(
      this.baseUrl + '/session',
      this.defaultFetchArgs('DELETE', null)
    );
    return response.json();
  }

  /**
   * Creates a new snippet.
   * @param input describes the snippet to create.
   */
  createSnippet(input: CreateSnippetInput): CreateSnippetOutput {
    // TODO: do the hard thing
    return {
      snippet: {
        id: 0,
        creatorId: 1,
        taxonomy: input.taxonomy,
        hidden: input.hidden,
        title: input.title,
        icon: input.icon,
        sharedBy: input.sharedBy,
        sharedOn: input.sharedOn,
        summary: input.summary,
        description: input.description,
        href: input.href,
        createdAt: new Date(),
        updatedAt: new Date(),
      }
    }
  }

  /**
   * Lists existing snippets.
   * @param input describes constraints in the list snippets call.
   */
  async listSnippets(input: ListSnippetInput): Promise<ListSnippetOutput> {
    const response = await fetch(
      this.baseUrl + `/snippets?taxonomy=${input.taxonomy}&page=${input.page}&showHidden=${input.showHidden}`,
      this.defaultFetchArgs('GET', null),
    );
    const output: ListSnippetOutput = await response.json();
    // this is super annoying but updates plain strings from json into
    // proper dates that can be reasoned about.
    output.snippets.forEach(snippet => parseSnippetDates(snippet));
    return output;
  }

  /**
   * Gets a single Snippet.
   * @param input the snippet to get.
   * @returns the snippet that was got.
   */
  async getSnippet(input: GetSnippetInput): Promise<GetSnippetOutput> {
    const response = await fetch(
      this.baseUrl + '/snippets/' + input.snippetId,
      this.defaultFetchArgs('GET', null),
    );
    const output: GetSnippetOutput = await response.json();
    parseSnippetDates(output.snippet);
    return output;
  }

  /**
   * Updates a single snippet.
   * @param input snippet input data.
   * @returns The result.
   */
  async updateSnippet(input: UpdateSnippetInput): Promise<UpdateSnippetOutput> {
    const response = await fetch(
      this.baseUrl + '/snippets/' + input.id,
      this.defaultFetchArgs('PUT', input),
    );
    const output: UpdateSnippetOutput = await response.json();
    return output;
  }

  defaultFetchArgs<T>(method: string, body: T): RequestInit {
    let args: RequestInit = {
      method: method,
      credentials: 'include',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
    };

    if (body != null) {
      const t: T = {} as T;
      console.log(t);
      args.body = JSON.stringify(body);
    }

    return args;
  }
}

function parseSnippetDates(snippet: Snippet): Snippet {
  snippet.createdAt = new Date(snippet.createdAt);
  snippet.updatedAt = new Date(snippet.updatedAt);
  snippet.sharedOn = new Date(snippet.sharedOn);
  return snippet;
}

/**
 * The initial state for the HttpClient. We don't need to mutate this,
 * so it won't have any reducer functions.
 */
const initialState: HttpClientProps = {
  baseUrl: '/api',
};

/**
 * Redux slice for the HttpClient properties.
 */
export const clientPropsSlice = createSlice({
  name: 'clientProps',
  initialState,
  reducers: {},
});

/**
 * Reducer which is handed off to Redux.
 */
export default clientPropsSlice.reducer;
