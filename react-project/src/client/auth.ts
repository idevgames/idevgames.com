export interface SessionIdentity {
    id: number;
    githubUserId: number;
    login: string;
}
export interface GetSessionInput { }
export interface GetSessionOutput {
    user?: SessionIdentity;
    permissions: string[];
}
export interface GetGithubAuthorizationUrlInput { }
export interface GetGithubAuthorizationUrlOutput {
    url: string;
}
export interface GetGithubCallbackInput {
    code: string;
}
export interface GetGithubCallbackOutput {
    user: SessionIdentity;
    permissions: string[];
}
export interface DeleteSessionInput { }
export interface DeleteSessionOutput { }