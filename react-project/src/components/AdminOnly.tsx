import { ReactNode } from 'react';
import { useAppSelector } from '../hooks';

export interface IAdminOnlyProps {
  children: ReactNode,
}

/**
 * It's like Farmers Only, but instead of hunky agricultural lads it's
 * just sweaty overweight engineers. So indeed they're nothing alike.
 */
export default function AdminOnly(props: IAdminOnlyProps) {
  const session = useAppSelector(state => state.session);

  if (!session.permissions.includes('admin')) {
    // this is something of a formality, the backend will reject edits
    // anyway.
    return <>
      <h1>Not authorized</h1>
      <p>
        Only editors may modify this site content.
      </p>
    </>;
  } else {
    return <>{props.children}</>;
  }
}
