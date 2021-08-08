import { useEffect, useState } from 'react';
import { NUM_GENOTYPE_GROUPS, SEED_SIZE } from '../shared/Constants';

// Separation between the big cells
const S1 = 3;
// Internal separation within the big cells
const S2 = 1;
// Separation between the top rows and the seed patch
const S3 = 8;

function fillStyle(intensity) {
    const boundedIntensity = Math.min(1, Math.max(0, Number(intensity)));

    return `rgb(${(1 - boundedIntensity) * 255}, ${boundedIntensity * 255}, 0)`;
}

function drawGenotype(ctx, genotype, plotSettings) {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    ctx.fillStyle = "#A0A0A0";
    ctx.fillRect(plotSettings.x0, plotSettings.y0, plotSettings.width, plotSettings.height);

    let index = 0;
    let x = 0;
    let y = 0;
    const numGroupBits = 2 * NUM_GENOTYPE_GROUPS;
    const w = plotSettings.cellSize * 2 + S2;

    while (index < numGroupBits) {
        ctx.fillStyle = fillStyle(genotype[index]);
        ctx.fillRect(x + plotSettings.x0 + S1, y + plotSettings.y0 + S1, w, w);

        index += 1;
        if (index % NUM_GENOTYPE_GROUPS === 0) {
            x = 0;
            y += w + S1;
        } else {
            x += w + S1;
        }
    }

    y += S3;
    const plotCell = function(index, x, y) {
        ctx.fillStyle = fillStyle(genotype[index]);
        ctx.fillRect(x + plotSettings.x0 + S1, y + plotSettings.y0 + S1, plotSettings.cellSize, plotSettings.cellSize);
    }

    let col = 0;
    while (index < genotype.length) {
        plotCell(index    , x, y);
        plotCell(index + 1, x + plotSettings.cellSize + S2, y);
        plotCell(index + 2, x, y + plotSettings.cellSize + S2);
        plotCell(index + 3, x + plotSettings.cellSize + S2, y + plotSettings.cellSize + S2);

        col += 1;
        index += 4;
        if (col % SEED_SIZE === 0) {
            x = 0;
            y += w + S1;
        } else {
            x += w + S1;
        }
    }
}

export function GenotypePlot({ genotype, plotId }) {
    const [plotSettings, setPlotSettings] = useState();

    useEffect(() => {
        if (plotSettings) {
            return;
        }

        const canvas = document.getElementById(plotId);
        const horizontalCellSpacing = (SEED_SIZE + 1) * S1;
        const horizontalCellSpace = canvas.width - horizontalCellSpacing;
        const verticalCellSpacing = (SEED_SIZE + 3) * S1 + S3;
        const verticalCellSpace = canvas.height - verticalCellSpacing;
        const bigCellMaxWidth = Math.floor(horizontalCellSpace / SEED_SIZE);
        const bigCellMaxHeight = Math.floor(verticalCellSpace / (SEED_SIZE + 2));
        const bigCellMaxSize = Math.min(bigCellMaxHeight, bigCellMaxWidth); 
        const cellSize = Math.floor((bigCellMaxSize - S2) / 2);
        const bigCellSize = cellSize * 2 + S2;
        const width = horizontalCellSpacing + SEED_SIZE * bigCellSize;
        const height = verticalCellSpacing + (SEED_SIZE + 2) * bigCellSize;
        const settings = {
            cellSize,
            x0: Math.floor((canvas.width - width) / 2),
            y0: Math.floor((canvas.height - height) / 2),
            width,
            height,
        }

        console.info({w: canvas.width, h: canvas.height, horizontalCellSpace, verticalCellSpace, bigCellSize});
        console.info("Plot settings:", settings);

        setPlotSettings(settings);
    }, [plotSettings, plotId]);

    useEffect(() => {
        const canvas = document.getElementById(plotId);
        const ctx = canvas.getContext('2d');

        if (genotype && plotSettings) {
            drawGenotype(ctx, genotype, plotSettings);
        }
    }, [plotSettings, plotId, genotype]);

    return (<div>
        <canvas id={plotId} height={200}></canvas>
    </div>);
}