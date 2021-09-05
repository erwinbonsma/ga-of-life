import React, { useContext, useEffect } from 'react';
import Button from 'react-bootstrap/Button';

// eslint-disable-next-line import/no-webpack-loader-syntax
import worker from 'workerize-loader!../workers/EaWorker';

export const ControlContext = React.createContext();

export function eaControlReducer(state, action) {
    console.info('eaReducer', { state, action });

    switch (action.type) {
        case 'initialize': return {
            settings: action.settings
        };
        case 'initialized': return {
            worker: action.worker,
            isRunning: false,
            executeStep: false,
            autoRun: true,
            runTime: 0,
        };
        case 'toggleAutoRun': return {
            ...state,
            autoRun: !state.autoRun,
        }
        case 'executeStep': return {
            ...state,
            executeStep: true,
        };
        case 'startStep': return {
            ...state,
            isRunning: true,
            executeStep: false,
        };
        case 'executedStep': return {
            ...state,
            isRunning: false,
            runTime: state.runTime + action.executionTime,
            eaState: action.eaState
        };
        case 'reset': return {
            ...state,
            destroy: true,
        };
        case 'executedReset': return undefined;
        default:
            console.error('Unexpected action:', action.type);
    }
}

export function EaControl() {
    const { eaControl, eaControlDispatch } = useContext(ControlContext);

    // console.info("eaControl =", eaControl);

    // Init EA in worker thread
    useEffect(() => {
        async function init(settings) {
            console.info("Setting worker");
            const eaWorker = new worker();
            await eaWorker.init(settings);
            eaControlDispatch({ type: 'initialized', worker: eaWorker });
        }

        if (eaControl.settings) {
            init(eaControl.settings);
        }
    }, [eaControl.settings, eaControlDispatch]);

    useEffect(() => {
        if (eaControl.worker && !eaControl.isRunning) {
            if (eaControl.executeStep || eaControl.autoRun) {
                eaControlDispatch({ type: 'startStep' });
                const startStep = new Date().getTime();

                eaControl.worker.step().then(eaState => {
                    const endStep = new Date().getTime();
                    eaControlDispatch({
                        type: 'executedStep',
                        executionTime: (endStep - startStep),
                        eaState 
                    });
                });
            }
        }
    }, [eaControl, eaControlDispatch]);

    useEffect(() => {
        if (eaControl.destroy) {
            console.info("Terminating worker");
            eaControl.worker.terminate();
            eaControlDispatch({ type: 'executedReset' });
        }
    }, [eaControl.destroy, eaControlDispatch]);

    const isRunning = eaControl.isRunning || eaControl.autoRun;
    return (
        <div>
            <Button onClick={() => eaControlDispatch({ type: 'reset' })} disabled={isRunning}>Reset</Button>
            <Button onClick={() => eaControlDispatch({ type: 'toggleAutoRun' })} >{eaControl.autoRun ? 'Pause' : 'Run'}</Button>
            <Button onClick={() => eaControlDispatch({ type: 'executeStep' })} disabled={isRunning}>Step</Button>
        </div>
    );
}