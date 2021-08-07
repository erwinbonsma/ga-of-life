import React, { useEffect, useState } from 'react';
import Button from 'react-bootstrap/Button';

// eslint-disable-next-line import/no-webpack-loader-syntax
import worker from 'workerize-loader!../workers/EaWorker';

export function EaRunner({ onStep }) {
    const [autoRun, setAutoRun] = useState(false);
    const [isRunning, setIsRunning] = useState(false);
    const [executeStep, setExecuteStep] = useState(false);
    const [eaRunner, setEaRunner] = useState();

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
            await eaWorker.init();    
            setEaRunner(eaWorker);
        }

        if (!eaRunner) {
            init();
        } else {
            return function cleanup() {
                eaRunner.terminate();
            }
        }
    }, [eaRunner, setEaRunner]);

    useEffect(() => {
        if (!isRunning) {
            if (executeStep || autoRun) {
                setExecuteStep(false);
                setIsRunning(true);

                eaRunner.step().then(results => {
                    setIsRunning(false);
                    onStep?.(results);
                });
            }
        }
    }, [isRunning, executeStep, autoRun, eaRunner, onStep]);

    return (
        <div>
            <Button onClick={onStartClick} disabled={isRunning || autoRun}>Run</Button>
            <Button onClick={onPauseClick} disabled={!autoRun}>Pause</Button>
            <Button onClick={onStepClick} disabled={isRunning || autoRun}>Step</Button>
        </div>
    );
}