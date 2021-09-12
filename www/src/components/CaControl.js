import React, { useContext, useEffect, useRef } from 'react';
import Button from 'react-bootstrap/Button';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { GRID_SIZE, SEED_SIZE } from '../shared/Constants';

const CELL_SIZE = 4;
const GRID_COLOR = "#CCCCCC";
const EMPTY_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const LIVED_COLOR = "#A0A0A0";

export const CaSettingsContext = React.createContext();
export const CaControlContext = React.createContext();

export const initialCaSettings = {
    borderWraps: false,
    gridSize: GRID_SIZE,
};
export const initialCaControlState = undefined;

export function caSettingsReducer(state, action) {
    switch (action.type) {
        case 'borderWraps': return {
            ...state, borderWraps: action.value
        };
        case 'gridSize': return {
            ...state, gridSize: action.value
        };
        default:
            console.error('Unexpected action:', action.type);
    }
}

let wasm;
async function wasmInit() {
    if (!wasm) {
        console.info("Loading CA wasm");

        wasm = await import('ga-of-life');
    }

    return wasm;
};

function drawGrid(ctx) {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= GRID_SIZE; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * GRID_SIZE + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= GRID_SIZE; j++) {
        ctx.moveTo(0,                               j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * GRID_SIZE + 1, j * (CELL_SIZE + 1) + 1);
    }
  
    ctx.stroke();
}

function drawCells(ctx, ca, onceAlive) {
    ctx.beginPath();
  
    for (let row = 0; row < GRID_SIZE; row++) {
        for (let col = 0; col < GRID_SIZE; col++) {
        
            if (ca.get(col, row)) {
                ctx.fillStyle = ALIVE_COLOR;
            } else if (onceAlive[col + row * GRID_SIZE]) {
                ctx.fillStyle = LIVED_COLOR;
            } else {
                ctx.fillStyle = EMPTY_COLOR;
            }
  
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
  
    ctx.stroke();
}

function drawContext(canvasRef) {
    return canvasRef?.current?.getContext('2d');
}

function updateCaStats(ca, onceAlive) {
    let numAlive = 0;
    for (let row = 0; row < GRID_SIZE; row++) {
        for (let col = 0; col < GRID_SIZE; col++) {
            if (ca.get(col, row)) {
                onceAlive[col + row * GRID_SIZE] = true;
                numAlive += 1;
            }
        }
    }

    return { numAlive, numOnceAlive: onceAlive.filter(x => x).length };
}

function executeStep(ca, onceAlive, ctx) {
    ca.step();
    const stats = updateCaStats(ca, onceAlive);
    drawCells(ctx, ca, onceAlive);

    return stats;
}

function seedCa(ca, seed) {
    ca.reset();

    const xy0 = (GRID_SIZE - SEED_SIZE) / 2;
    for (let x = 0; x < SEED_SIZE; x++) {
        for (let y = 0; y < SEED_SIZE; y++) {
            if (seed.charAt(x + y * SEED_SIZE) !== '0') {
                ca.set(x + xy0, y + xy0);
            }
        }
    }
}

export function caControlReducer(state, action) {
    switch (action.type) {
        case 'initialize': return initialCaControlState;
        case 'initialized': return {
            ca: action.ca,
            seed: action.seed,
            numSteps: 0,
            autoRun: false,
        };
        case 'toggleAutoRun': return {
            ...state,
            autoRun: !state.autoRun,
        }
        case 'executedStep': return {
            ...state,
            numSteps: state.numSteps + 1,
            caStats: action.stats
        };
        default:
            console.error('Unexpected action:', action.type);
    }
}

export function CaControl({ seed }) {
    const { caControl, caControlDispatch } = useContext(CaControlContext);
    const { caSettings } = useContext(CaSettingsContext);
    const onceAliveRef = useRef(new Array(GRID_SIZE * GRID_SIZE));
    const canvasRef = useRef(null);

    const clearOnceAlive = () => {
        onceAliveRef.current.forEach((_, i, a) => { a[i] = 0; });
    }

    const reset = (ca) => {
        caControlDispatch({ type: 'initialize' });
    }

    const step = (ca) => {
        const stats = executeStep(ca, onceAliveRef.current, drawContext(canvasRef));
        caControlDispatch({ type: 'executedStep', stats });
    }

    useEffect(() => {
        async function init() {
            const wasm = await wasmInit();
            const ca = new wasm.GameOfLife(caSettings.gridSize, caSettings.gridSize, caSettings.borderWraps);
            seedCa(ca, seed);
            clearOnceAlive();
            caControlDispatch({ type: 'initialized', ca, seed });
        }

        if (!caControl?.ca || caControl.seed !== seed) {
            init();
        } else {
            drawGrid(drawContext(canvasRef));
            drawCells(drawContext(canvasRef), caControl.ca, onceAliveRef.current);
        }
    }, [seed, caControl?.ca, caControl?.seed, caSettings, caControlDispatch]);

    useEffect(() => {
        if (caControl?.autoRun) {
            const timer = setTimeout(() => {
                step(caControl.ca);
            }, 10);

            return function cleanup() {
                clearTimeout(timer);
            }
        }
    });

    return (<Container>
        <Row className="ButtonRow">
            <Col>
                <Button onClick={() => caControlDispatch({ type: 'toggleAutoRun' })} disabled={!caControl || caControl.autoRun}>Play</Button>{' '}
                <Button onClick={() => caControlDispatch({ type: 'toggleAutoRun' })} disabled={!caControl || !caControl.autoRun}>Pause</Button>{' '}
                <Button onClick={() => step(caControl.ca)} disabled={!caControl || caControl.autoRun}>Step</Button>{' '}
                <Button onClick={() => reset(caControl.ca)} disabled={!caControl || caControl.autoRun}>Reset</Button>
            </Col>
        </Row>
        <Row>
            <Col>
                <canvas ref={canvasRef}
                    width={(CELL_SIZE + 1) * GRID_SIZE}
                    height={(CELL_SIZE + 1) * GRID_SIZE}></canvas>
            </Col>
        </Row>
    </Container>);
}