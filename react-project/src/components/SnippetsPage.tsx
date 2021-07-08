import { useParams } from "react-router-dom";
import { useQuery } from "../hooks";
import SnippetList from "./SnippetList";

type RouteParams = {
  taxonomy: string,
  page: string,
};

export default function SnippetsPage() {
  let route = useParams<RouteParams>();
  let query = useQuery();
  let taxonomy = route.taxonomy;
  let page = parseInt(route.page);
  let showHidden = (query.get('showHidden') === 'true');
  return <SnippetList taxonomy={taxonomy} page={page} showPager={true} showHidden={showHidden} />;
}
