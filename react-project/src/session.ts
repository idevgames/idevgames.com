import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { SessionIdentity } from './client/auth';

export interface SessionState {
    sessionIdentity: SessionIdentity | null,
    permissions: string[],
};

const initialState: SessionState = {
    sessionIdentity: null,
    permissions: []
};

export const sessionSlice = createSlice({
    name: 'session',
    initialState,
    reducers: {
        setSession: (state, action: PayloadAction<SessionState>) => {
            state.sessionIdentity = action.payload.sessionIdentity;
            state.permissions = action.payload.permissions;
        },
    },
});

export const { setSession } = sessionSlice.actions;
export default sessionSlice.reducer;
