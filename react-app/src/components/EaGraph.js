import { useEffect, useState } from 'react';

const historyLen = 10;

export function EaGraph({ eaState }) {
    const [history, setHistory] = useState([]);

    useEffect(() => {
        if (eaState?.generations !== history[history.length - 1]?.generations) {
            if (history.length < historyLen) {
                setHistory([
                    ...history, eaState
                ]);
            } else {
                setHistory([
                    ...history.slice(1, historyLen), eaState
                ]);
            }
        }
    }, [eaState, history])

    return (
        <ul>
            {history.map(state => 
                <li key={state.generations}>{state.generations}. {state.maxFitness}</li>
            )}
        </ul>
    )
}