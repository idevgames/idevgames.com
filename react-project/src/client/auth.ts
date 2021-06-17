export interface GetSessionInput { }
export interface GetSessionOutput {
    github_user_id: number;
    permissions: string[];
}
export interface GetGithubAuthorizationUrlInput { }
export interface GetGithubAuthorizationUrlOutput {
    url: string;
}
export interface GetGithubCallbackInput {
    code: string;
}
export interface GetGithubCallbackOutput { }
export interface DestroySessionInput { }
export interface DestroySessionOutput { }