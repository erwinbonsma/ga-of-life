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

export function CaRunner({ seed }) {
    const [ca, setCa] = useState();
    const [toggled, setToggled] = useState();

    const clearToggled = () => {
        setToggled(new Array(GRID_SIZE * GRID_SIZE));
    }
    const updateToggled = () => {
        for (let row = 0; row < GRID_SIZE; row++) {
            for (let col = 0; col < GRID_SIZE; col++) {
                if (ca.get(col, row)) {
                    toggled[col + row * GRID_SIZE] = true;
                }
            }
        }
    }

    const onSeedClick = () => {
        ca.reset();
        clearToggled();

        const xy0 = (GRID_SIZE - SEED_SIZE) / 2;
        for (let x = 0; x < SEED_SIZE; x++) {
            for (let y = 0; y < SEED_SIZE; y++) {
                if (seed.charAt(x + y * (SEED_SIZE + 1)) !== ' ') {
                    ca.set(x + xy0, y + xy0, true);
                }
            }
        }

        drawCells(drawContext(), ca, toggled);
    }

    const onStepClick = () => {
        ca.step();
        updateToggled();
        drawCells(drawContext(), ca, toggled);
    }

    useEffect(() => {
        async function init() {
            console.info("Loading CA wasm");
            setCa(await wasmInit());
        }

        if (!ca) {
            clearToggled();
            init();
        } else {
            const ctx = drawContext();
            drawGrid(ctx);
            drawCells(ctx, ca, toggled);
        }
    }, [ca, toggled]);

    return (<div>
        <Button onClick={onSeedClick} disabled={!ca}>Seed</Button>
        <Button onClick={onStepClick} disabled={!ca}>Grow</Button>
        <canvas id="ca-canvas"
            width={(CELL_SIZE + 1) * GRID_SIZE}
            height={(CELL_SIZE + 1) * GRID_SIZE}></canvas>
    </div>);
}