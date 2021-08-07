import { useEffect, useState } from 'react';
import { NUM_GENOTYPE_GROUPS, SEED_SIZE } from '../shared/Constants';

function drawGenotype(ctx, genotype, cellSize) {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);

    let index = 0;
    let x = 0;
    let y = 0;
    const numGroupBits = 2 * NUM_GENOTYPE_GROUPS;
    const w = cellSize * 2 + 1;

    while (index < numGroupBits) {
        if (genotype[index] === '1') {
            ctx.fillStyle = "#00FF00";
        } else {
            ctx.fillStyle = "#000000";
        }

        ctx.fillRect(x, y, w, w);

        index += 1;
        if (index % NUM_GENOTYPE_GROUPS === 0) {
            x = 0;
            y += w + 2;
        } else {
            x += w + 2;
        }
    }

    y += cellSize;
    const plotCell = function(index, x, y) {
        if (genotype[index] === '1') {
            ctx.fillStyle = "#00FF00";
        } else {
            ctx.fillStyle = "#000000";
        }

        ctx.fillRect(x, y, cellSize, cellSize);
    }

    let col = 0;
    while (index < genotype.length) {
        plotCell(index    , x, y);
        plotCell(index + 1, x + cellSize + 1, y);
        plotCell(index + 2, x, y + cellSize + 1);
        plotCell(index + 3, x + cellSize + 1, y + cellSize + 1);

        col += 1;
        index += 4;
        if (col % SEED_SIZE === 0) {
            x = 0;
            y += w + 2;
        } else {
            x += w + 2;
        }
    }
}

export function GenotypePlot({ genotype, plotId }) {
    const [cellSize, setCellSize] = useState(4);

    useEffect(() => {
        const canvas = document.getElementById(plotId);
        const ctx = canvas.getContext('2d');

        if (genotype) {
            drawGenotype(ctx, genotype, cellSize);
        }
    });

    return (<div>
        <canvas id={plotId} width="100" height="120" ></canvas>
    </div>);
}