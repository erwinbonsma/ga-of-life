import { useEffect, useRef, useReducer } from 'react';
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

let wasmCa;
export async function wasmInit() {
    if (!wasmCa) {
        const wasm = await import('ga-of-life');

        wasmCa = new wasm.GameOfLife(GRID_SIZE, GRID_SIZE);
    }

    return wasmCa;
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

function drawCells(ctx, ca, toggled) {
    ctx.beginPath();
  
    for (let row = 0; row < GRID_SIZE; row++) {
        for (let col = 0; col < GRID_SIZE; col++) {
        
            if (ca.get(col, row)) {
                ctx.fillStyle = ALIVE_COLOR;
            } else if (toggled[col + row * GRID_SIZE]) {
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

function updateToggled(ca, toggled) {
    for (let row = 0; row < GRID_SIZE; row++) {
        for (let col = 0; col < GRID_SIZE; col++) {
            if (ca.get(col, row)) {
                toggled[col + row * GRID_SIZE] = true;
            }
        }
    }
}

function executeStep(ca, toggled, ctx) {
    ca.step();
    updateToggled(ca, toggled);
    drawCells(ctx, ca, toggled);
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
    console.info('caReducer', { state, action });

    switch (action.type) {
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
            numSteps: state.numSteps + 1
        };
        default:
            console.error('Unexpected action:', action.type);
    }
}

export function CaControl({ seed }) {
    const [caControl, caControlDispatch] = useReducer(caControlReducer);
    const toggledRef = useRef(new Array(GRID_SIZE * GRID_SIZE));
    const canvasRef = useRef(null);

    const clearToggled = () => {
        toggledRef.current.forEach((_, i, a) => { a[i] = 0; });
    }

    const reset = (ca) => {
        clearToggled();
        seedCa(ca, seed);

        drawCells(drawContext(canvasRef), ca, toggledRef.current);
    }

    const step = (ca) => {
        executeStep(ca, toggledRef.current, drawContext(canvasRef));
    }

    useEffect(() => {
        async function init() {
            console.info("Loading CA wasm");

            const ca = await wasmInit();
            seedCa(ca, seed);
            caControlDispatch({ type: 'initialized', ca, seed });
        }

        if (!caControl?.ca || caControl.seed !== seed) {
            init();
        } else {
            drawGrid(drawContext(canvasRef));
            drawCells(drawContext(canvasRef), caControl.ca, toggledRef.current);
        }
    }, [seed, caControl?.ca, caControl?.seed]);

    useEffect(() => {
        if (caControl?.autoRun) {
            const timer = setTimeout(() => {
                executeStep(caControl.ca, toggledRef.current, drawContext(canvasRef));
                // Trigger next update
                caControlDispatch({ type: 'executedStep' });
            }, 10);

            return function cleanup() {
                clearTimeout(timer);
            }
        }
    });

    return (<Container>
        <Row>
            <Col>
                <Button onClick={() => reset(caControl.ca)} disabled={!caControl || caControl.autoRun}>Reset</Button>
                <Button onClick={() => caControlDispatch({ type: 'toggleAutoRun' })} disabled={!caControl}>{caControl?.autoRun ? 'Pause' : 'Play'}</Button>
                <Button onClick={() => step(caControl.ca)} disabled={!caControl || caControl.autoRun}>Step</Button>
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