import React, { useState } from 'react';
import Button from 'react-bootstrap/Button';

export function EaRunner() {
    const [numSteps, setNumSteps] = useState(0);
    const [isRunning, setIsRunning] = useState(false);

    const onStartClick = () => {
        setIsRunning(true);
    }
    const onPauseClick = () => {
        setIsRunning(false);
    }
    const onStepClick = () => {
        setNumSteps(numSteps + 1);
    }

    return (
        <div>
            <Button onClick={onStartClick} disabled={isRunning}>Run</Button>
            <Button onClick={onPauseClick} disabled={!isRunning}>Pause</Button>
            <Button onClick={onStepClick} disabled={isRunning}>Step</Button>
            <p>Step: {numSteps}</p>
        </div>
    );
}