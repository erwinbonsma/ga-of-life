import React, { useEffect, useState } from 'react';
import Button from 'react-bootstrap/Button';

// eslint-disable-next-line import/no-webpack-loader-syntax
import worker from 'workerize-loader!../workers/EaWorker'; 

export function EaRunner() {
    const [numSteps, setNumSteps] = useState(0);
    const [isRunning, setIsRunning] = useState(false);
    const [eaRunner, setEaRunner] = useState();

    const onStartClick = () => {
        setIsRunning(true);
    }
    const onPauseClick = () => {
        setIsRunning(false);
    }
    const onStepClick = () => {
        setNumSteps(numSteps + 1);
        setIsRunning(true);
        eaRunner.step().then(results => {
            console.info(results);
            setIsRunning(false);
        });
    }

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

    return (
        <div>
            <Button onClick={onStartClick} disabled={isRunning}>Run</Button>
            <Button onClick={onPauseClick} disabled={!isRunning}>Pause</Button>
            <Button onClick={onStepClick} disabled={isRunning}>Step</Button>
            <p>Step: {numSteps}</p>
        </div>
    );
}