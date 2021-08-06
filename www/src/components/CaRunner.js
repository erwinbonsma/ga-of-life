import { useEffect, useState } from 'react';
import Button from 'react-bootstrap/Button';

const GRID_SIZE = 64;
const SEED_SIZE = 8;

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

function drawContext() {
    const canvas = document.getElementById("ca-canvas");
    return canvas.getContext('2d');
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

function executeStep(ca, toggled) {
    ca.step();
    updateToggled(ca, toggled);
    drawCells(drawContext(), ca, toggled);
}

function seedCa(ca, seed) {
    ca.reset();

    const xy0 = (GRID_SIZE - SEED_SIZE) / 2;
    for (let x = 0; x < SEED_SIZE; x++) {
        for (let y = 0; y < SEED_SIZE; y++) {
            if (seed.charAt(x + y * (SEED_SIZE + 1)) !== ' ') {
                ca.set(x + xy0, y + xy0);
            }
        }
    }
}

export function CaRunner({ seed }) {
    const [ca, setCa] = useState();
    const [toggled, setToggled] = useState(new Array(GRID_SIZE * GRID_SIZE));
    const [autoPlay, setAutoPlay] = useState();
    const [scheduleStep, setScheduleStep] = useState(0);

    const clearToggled = () => {
        setToggled(new Array(GRID_SIZE * GRID_SIZE));
    }

    const onSeedClick = () => {
        clearToggled();
        seedCa(ca, seed);

        drawCells(drawContext(), ca, toggled);
    }

    const onStepClick = () => {
        executeStep(ca, toggled);
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
            const ctx = drawContext();
            drawGrid(ctx);
        }
    }, [ca, toggled]);

    useEffect(() => {
        if (autoPlay) {
            const timer = setTimeout(() => {
                executeStep(ca, toggled);
                // Trigger next update
                setScheduleStep(scheduleStep + 1);
            }, 10);

            return function cleanup() {
                clearTimeout(timer);
            }
        }
    });

    return (<div>
        <Button onClick={onSeedClick} disabled={!ca}>Seed</Button>
        <Button onClick={onStepClick} disabled={!ca || autoPlay}>Step</Button>
        <Button onClick={onTogglePlayClick} disabled={!ca || autoPlay}>Play</Button>
        <Button onClick={onTogglePlayClick} disabled={!(ca && autoPlay)}>Pause</Button>
        <canvas id="ca-canvas"
            width={(CELL_SIZE + 1) * GRID_SIZE}
            height={(CELL_SIZE + 1) * GRID_SIZE}></canvas>
    </div>);
}