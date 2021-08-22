import { useEffect, useRef, useState } from 'react';
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

export function CaRunner({ seed }) {
    const [ca, setCa] = useState();
    const [toggled, _] = useState(new Array(GRID_SIZE * GRID_SIZE));
    const [autoPlay, setAutoPlay] = useState();
    const [scheduleStep, setScheduleStep] = useState(0);
    const canvasRef = useRef(null);

    const clearToggled = () => {
        toggled.forEach((_, i, a) => { a[i] = 0; });
    }

    const onSeedClick = () => {
        clearToggled();
        seedCa(ca, seed);

        drawCells(drawContext(canvasRef), ca, toggled);
    }

    const onStepClick = () => {
        executeStep(ca, toggled, drawContext(canvasRef));
    }
    const onTogglePlayClick = () => {
        setAutoPlay(!autoPlay);
    }

    useEffect(() => {
        async function init() {
            console.info("Loading CA wasm");
            setCa(await wasmInit());
        }

        if (!ca) {
            init();
        } else {
            drawGrid(drawContext(canvasRef));
        }
    }, [ca, toggled]);

    useEffect(() => {
        if (autoPlay) {
            const timer = setTimeout(() => {
                executeStep(ca, toggled, drawContext(canvasRef));
                // Trigger next update
                setScheduleStep(scheduleStep + 1);
            }, 10);

            return function cleanup() {
                clearTimeout(timer);
            }
        }
    });

    return (<Container>
        <Row>
            <Col>
                <Button onClick={onSeedClick} disabled={!ca || autoPlay}>Seed</Button>
                <Button onClick={onTogglePlayClick} disabled={!ca || autoPlay}>Play</Button>
                <Button onClick={onTogglePlayClick} disabled={!(ca && autoPlay)}>Pause</Button>
                <Button onClick={onStepClick} disabled={!ca || autoPlay}>Step</Button>
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