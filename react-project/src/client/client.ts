import {
  CreateSnippetInput, CreateSnippetOutput, ListSnippetInput, ListSnippetOutput,
  Snippet
} from "./snippets";

/**
 * Describes how to interact with the iDevGames server.
 */
export interface Client {
  /**
   * Creates a new snippet.
   * @param input describes the snippet to create.
   */
  createSnippet(input: CreateSnippetInput): CreateSnippetOutput;
  /**
   * Lists existing snippets.
   * @param input describes constraints in the list snippets call.
   */
  listSnippets(input: ListSnippetInput): ListSnippetOutput;
}

/**
 * A client which interacts with the iDevGames server.
 */
export class HttpClient implements Client {
  baseUrl: string;
  /**
   * Creates a new HttpClient.
   */
  constructor() {
    if (process.env.NODE_ENV === 'production') {
      this.baseUrl = 'https://www.idevgames.com';
    } else {
      this.baseUrl = 'http://localhost:4000';
    }
  }
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
        createdAt: Date(),
        updatedAt: Date(),
      }
    }
  }
  listSnippets(input: ListSnippetInput): ListSnippetOutput {
    // TODO: do the hard thing
    const referenceSnippets: {[index: string]:Snippet[]} = {
      "links": [
        {
          id: 1,
          creatorId: 1,
          taxonomy: "links",
          hidden: false,
          title: "www.blender.org: Blender 2.82 released",
          icon: "blender.png",
          sharedBy: "mysteriouspants",
          sharedOn: "2020-02-14",
          summary: "The Blender Foundation has released version 2.82 of their incredible free and open-source 3D-modelling and animation software, including features such as improved fluid and smoke simulation.",
          description: "",
          href: "https://www.blender.org/press/blender-2-82-released/",
          createdAt: "sometime",
          updatedAt: "sometime",
        }
      ],
      "udevgames": [
        {
          id: 2,
          creatorId: 1,
          taxonomy: "udevgames",
          hidden: false,
          title: "uDevGames 2021Q1",
          icon: "itchio.png",
          sharedBy: "mysteriouspants",
          sharedOn: "2021-01-23",
          summary: "A quarter-long, freeform, hobbyist game jam.",
          description: "",
          href: "https://itch.io/jam/udevgames-2021q1",
          createdAt: "sometime",
          updatedAt: "sometime",
        }
      ]
    };
    return {
      snippets: referenceSnippets[input.taxonomy],
      nextPage: 0,
      totalPages: 1,
    };
  }
}
