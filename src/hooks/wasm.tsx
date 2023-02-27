import { useEffect, useReducer } from 'react';
import { Chess } from '../../wasm/pkg/chess_wasm.js';

let chess = {} as Chess;
try {
  chess = new Chess("");
} catch(e) {
  console.log(e);
}

export const useChess = (query) => {
  const initialState = {
    status: 'idle',
    error: null,
    data: [],
  };

  const [state, dispatch] = useReducer((state, action) => {
    switch (action.type) {
      case 'FETCHING':
        return { ...initialState, status: 'fetching' };
      case 'FETCHED':
        return { ...initialState, status: 'fetched', data: action.payload };
      case 'FETCH_ERROR':
        return { ...initialState, status: 'error', error: action.payload };
      default:
        return state;
    }
  }, initialState);

  useEffect(() => {
    if (!chess) {
      dispatch({ type: "FETCH_ERROR", payload: "WASM NOT ENABLED" });
    }
    if (!query) return;

    const fetchData = async () => {
      dispatch({ type: 'FETCHING' });

      try {
        const sleep = ms => new Promise(resolve => setTimeout(resolve, ms))
        await sleep(0);

        const pwasm = await chess;
        const calc = await pwasm.runCalculation(query);

        const data = calc;

        dispatch({ type: "FETCHED", payload: data });
      } catch (e) {
        dispatch({type: 'FETCH_ERROR', payload: e});
      }
    };

    fetchData();
  }, [query]);

  return state;
};
