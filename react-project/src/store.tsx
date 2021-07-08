import { configureStore } from '@reduxjs/toolkit';
import clientPropsReducer from './client/client';
import sessionReducer from './session';

const store = configureStore({
  reducer: {
    clientProps: clientPropsReducer,
    session: sessionReducer,
  }
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export default store;
