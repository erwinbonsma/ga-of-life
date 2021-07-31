import React, { useEffect, useState } from 'react';
import Button from 'react-bootstrap/Button';
import worker from 'workerize-loader!../workers/EaWorker'; // eslint-disable-line import/no-webpack-loader-syntax

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
        eaRunner.expensive(500).then(count => {
            console.info(`Looped ${count} times`);
            setIsRunning(false);
        });
    }

    useEffect(() => {
        console.info("Setting worker");
        setEaRunner(new worker());

        return function cleanup() {
            // TODO
        }
    }, [setEaRunner]);

    return (
        <div>
            <Button onClick={onStartClick} disabled={isRunning}>Run</Button>
            <Button onClick={onPauseClick} disabled={!isRunning}>Pause</Button>
            <Button onClick={onStepClick} disabled={isRunning}>Step</Button>
            <p>Step: {numSteps}</p>
        </div>
    );
}