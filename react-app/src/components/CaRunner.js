import { useEffect, useState } from 'react';
import Button from 'react-bootstrap/Button';

const GRID_SIZE = 64;
const SEED_SIZE = 8;

const CELL_SIZE = 4;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

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

function drawCells(ctx, ca) {
    ctx.beginPath();
  
    for (let row = 0; row < GRID_SIZE; row++) {
      for (let col = 0; col < GRID_SIZE; col++) {
        ctx.fillStyle = ca.get(col, row)
          ? ALIVE_COLOR
          : DEAD_COLOR;
  
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

function drawCa(ca) {
    const canvas = document.getElementById("ca-canvas");
    const ctx = canvas.getContext('2d');

    drawGrid(ctx);
    drawCells(ctx, ca);
}

export function CaRunner({ seed }) {
    const [ca, setCa] = useState();

    const onSeedClick = () => {
        ca.reset();

        const c0 = (GRID_SIZE - SEED_SIZE) / 2;
        for (let x = 0; x < SEED_SIZE; x++) {
            for (let y = 0; y < SEED_SIZE; y++) {
                if (seed.charAt(x + y * (SEED_SIZE + 1)) !== ' ') {
                    ca.set(x + c0, y + c0, true);
                }
            }
        }

        drawCa(ca);
    }

    const onStepClick = () => {
        ca.step();
        drawCa(ca);
    }

    useEffect(() => {
        async function init() {
            console.info("Loading CA wasm");
            setCa(await wasmInit());
        }

        if (!ca) {
            init();
        } else {
            ca.set(1, 0, true);
            ca.set(2, 1, true);
            ca.set(0, 2, true);
            ca.set(1, 2, true);
            ca.set(2, 2, true);

            drawCa(ca);
        }
    }, [ca]);

    return (<div>
        <Button onClick={onSeedClick} disabled={!ca}>Seed</Button>
        <Button onClick={onStepClick} disabled={!ca}>Grow</Button>
        <canvas id="ca-canvas" width={CELL_SIZE * GRID_SIZE} height={CELL_SIZE * GRID_SIZE}></canvas>
    </div>);
}