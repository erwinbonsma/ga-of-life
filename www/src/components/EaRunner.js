import React, { useContext, useEffect, useState } from 'react';
import Button from 'react-bootstrap/Button';

import { initialSettings, SettingsContext } from './EaSettings';

// eslint-disable-next-line import/no-webpack-loader-syntax
import worker from 'workerize-loader!../workers/EaWorker';

export function EaRunner({ onStep }) {
    const { settings } = useContext(SettingsContext);
    const [autoRun, setAutoRun] = useState(false);
    const [isRunning, setIsRunning] = useState(false);
    const [executeStep, setExecuteStep] = useState(false);
    const [eaRunner, setEaRunner] = useState();
    const [runTime, setRunTime] = useState(0);

    const onResetClick = () => {
        eaRunner.reset(settings);
        setRunTime(0);

        // Notify observers that EA was reset
        onStep?.(null);
    }
    const onStartClick = () => {
        setAutoRun(true);
    }
    const onPauseClick = () => {
        setAutoRun(false);
    }
    const onStepClick = () => {
        setExecuteStep(true);
    }

    // Init EA in worker thread
    useEffect(() => {
        async function init() {
            console.info("Setting worker");
            const eaWorker = new worker();
            await eaWorker.init(initialSettings);
            setEaRunner(eaWorker);
        }

        if (!eaRunner) {
            init();
        } else {
            return function cleanup() {
                eaRunner.terminate();
            }
        }
    }, [eaRunner]);

    useEffect(() => {
        if (!isRunning) {
            if (executeStep || autoRun) {
                setExecuteStep(false);
                setIsRunning(true);
                const startStep = new Date().getTime();

                eaRunner.step().then(results => {
                    const endStep = new Date().getTime();
                    const newRunTime = runTime + (endStep - startStep);
                    setIsRunning(false);
                    setRunTime(runTime => runTime + (endStep - startStep));
                    onStep?.({ ...results, runTime: Math.round(newRunTime / 1000) });
                });
            }
        }
    }, [isRunning, executeStep, autoRun, eaRunner, runTime, onStep]);

    return (
        <div>
            <Button onClick={onResetClick} disabled={isRunning || autoRun}>Reset</Button>
            <Button onClick={onStartClick} disabled={isRunning || autoRun}>Run</Button>
            <Button onClick={onPauseClick} disabled={!autoRun}>Pause</Button>
            <Button onClick={onStepClick} disabled={isRunning || autoRun}>Step</Button>
        </div>
    );
}