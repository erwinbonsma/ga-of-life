import React, { useCallback, useContext, useEffect, useRef } from 'react';
import Button from 'react-bootstrap/Button';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { MAX_GRID_SIZE, SEED_SIZE } from '../shared/Constants';
import { bound } from '../shared/utils';

const GRID_COLOR = "#CCCCCC";
const EMPTY_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const LIVED_COLOR = "#A0A0A0";

export const CaSettingsContext = React.createContext();
export const CaControlContext = React.createContext();

export const initialCaSettings = {
    borderWraps: false,
    gridSize: 64,
};
export const initialCaControlState = undefined;

export function caSettingsReducer(state, action) {
    switch (action.type) {
        case 'borderWraps': return {
            ...state, borderWraps: action.value
        };
        case 'gridSize': return {
            ...state, gridSize: bound(action.value, 32, MAX_GRID_SIZE)
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

export function caControlReducer(state, action) {
    switch (action.type) {
        case 'initialize': return initialCaControlState;
        case 'initializing': return {
            ca: action.ca,
            seed: action.seed,
            autoRun: false,
        };
        case 'initialized': return {
            ...state,
            numSteps: 0,
            caStats: action.stats,
        }
        case 'toggleAutoRun': return {
            ...state,
            autoRun: !state.autoRun,
        }
        case 'executedStep': return {
            ...state,
            numSteps: state.numSteps + 1,
            caStats: action.stats,
        };
        default:
            console.error('Unexpected action:', action.type);
    }
}

export function CaControl({ seed }) {
    const { caControl, caControlDispatch } = useContext(CaControlContext);
    const { caSettings } = useContext(CaSettingsContext);
    const onceAliveRef = useRef(new Array(caSettings.gridSize * caSettings.gridSize));
    const canvasRef = useRef(null);

    const ctx = canvasRef.current?.getContext('2d');
    const ca = caControl?.ca;
    const onceAlive = onceAliveRef.current;
    const gridWidth = caSettings.gridSize;
    const gridHeight = caSettings.gridSize;
    const cellSize = Math.max(4, Math.round(12 * 32 / Math.max(gridWidth, gridHeight)));

    const seedCa = useCallback(
        // Taking ca as arg, as it will be newly set
        (ca) => {
            ca.reset();

            const x0 = (gridWidth - SEED_SIZE) / 2;
            const y0 = (gridHeight - SEED_SIZE) / 2;
            for (let x = 0; x < SEED_SIZE; x++) {
                for (let y = 0; y < SEED_SIZE; y++) {
                    if (seed.charAt(x + y * SEED_SIZE) !== '0') {
                        ca.set(x + x0, y + y0);
                    }
                }
            }
        },
        [seed, gridWidth, gridHeight]
    )

    const drawGrid = useCallback(
        () => {
            ctx.beginPath();
            ctx.strokeStyle = GRID_COLOR;

            // Vertical lines.
            for (let i = 0; i <= gridWidth; i++) {
                ctx.moveTo(i * (cellSize + 1) + 1, 0);
                ctx.lineTo(i * (cellSize + 1) + 1, (cellSize + 1) * gridHeight + 1);
            }

            // Horizontal lines.
            for (let j = 0; j <= gridHeight; j++) {
                ctx.moveTo(0,                              j * (cellSize + 1) + 1);
                ctx.lineTo((cellSize + 1) * gridWidth + 1, j * (cellSize + 1) + 1);
            }

            ctx.stroke();
        },
        [ctx, gridWidth, gridHeight, cellSize]
    )

    const drawCells = useCallback(
        () => {
            ctx.beginPath();
            for (let row = 0; row < gridHeight; row++) {
                for (let col = 0; col < gridWidth; col++) {

                    if (ca.get(col, row)) {
                        ctx.fillStyle = ALIVE_COLOR;
                    } else if (onceAlive[col + row * gridWidth]) {
                        ctx.fillStyle = LIVED_COLOR;
                    } else {
                        ctx.fillStyle = EMPTY_COLOR;
                    }

                    ctx.fillRect(
                        col * (cellSize + 1) + 1,
                        row * (cellSize + 1) + 1,
                        cellSize,
                        cellSize
                    );
                }
            }

            ctx.stroke();
        },
        [ctx, ca, gridWidth, gridHeight, cellSize, onceAlive]
    );

    const updateCaStats = useCallback(
        () => {
            let numAlive = 0;
            for (let row = 0; row < gridHeight; row++) {
                for (let col = 0; col < gridWidth; col++) {
                    if (ca.get(col, row)) {
                        onceAlive[col + row * gridWidth] = true;
                        numAlive += 1;
                    }
                }
            }

            return { numAlive, numOnceAlive: onceAlive.filter(x => x).length };
        },
        [gridWidth, gridHeight, ca, onceAlive]
    )

    const step = useCallback(
        () => {
            ca.step();
            const stats = updateCaStats();
            drawCells();

            caControlDispatch({ type: 'executedStep', stats });
        },
        [ca, drawCells, caControlDispatch, updateCaStats]
    )

    const clearOnceAlive = useCallback(
        () => {
            onceAlive.forEach((_, i, a) => { a[i] = 0; });
        },
        [onceAlive]
    );

    const reset = useCallback(
        () => {
            caControlDispatch({ type: 'initialize' });
        },
        [caControlDispatch]
    );

    // Multi-step initialization
    useEffect(() => {
        async function init() {
            const wasm = await wasmInit();
            const ca = new wasm.GameOfLife(caSettings.gridSize, caSettings.gridSize, caSettings.borderWraps);
            seedCa(ca);
            clearOnceAlive();
            caControlDispatch({ type: 'initializing', ca, seed });
        }

        if (!caControl?.ca || caControl.seed !== seed) {
            init();
        } else if (!caControl.caStats) {
            const stats = updateCaStats();

            caControlDispatch({ type: 'initialized', stats });
        } else {
            drawGrid();
            drawCells();
        }
    }, [
        seed, caControl?.ca, caControl?.seed, caControl?.caStats, caSettings, caControlDispatch,
        seedCa, clearOnceAlive, updateCaStats, drawGrid, drawCells
    ]);

    useEffect(() => {
        if (caControl?.autoRun) {
            const timer = setTimeout(() => {
                step();
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
                <Button onClick={() => step()} disabled={!caControl || caControl.autoRun}>Step</Button>{' '}
                <Button onClick={() => reset()} disabled={!caControl || caControl.autoRun}>Reset</Button>
            </Col>
        </Row>
        <Row>
            <Col>
                <canvas ref={canvasRef}
                    width={(cellSize + 1) * caSettings.gridSize + 1}
                    height={(cellSize + 1) * caSettings.gridSize + 1}></canvas>
            </Col>
        </Row>
    </Container>);
}