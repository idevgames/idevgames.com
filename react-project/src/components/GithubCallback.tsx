import { HttpClient } from "../client/client";
import { useAppDispatch, useAppSelector, useQuery } from "../hooks";
import { setSession } from "../session";

export default function GithubCallback(_props: {}) {
    let query = useQuery();
    let code = query.get('code')!;
    let client = new HttpClient(useAppSelector(state => state.clientProps));
    let dispatch = useAppDispatch();

    client.getGithubCallback({ code })
        .then(output => {
            // this will cause the app to re-paint
            dispatch(setSession({
                sessionIdentity: output.user,
                permissions: output.permissions,
            }));
            // TODO: reroute to some kind of "hooray we did it" page
        })
        .catch(oops => {
            // TODO: redirect to an error page, i guess
            console.log('Failed to process callback with error', oops);
        });

    return <span>Logging you in...</span>
}
