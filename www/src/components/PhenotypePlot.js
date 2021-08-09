import { useEffect, useState } from 'react';
import { SEED_SIZE } from '../shared/Constants';

// Separation between the cells
const S1 = 3;

function fillStyle(intensity) {
    const boundedIntensity = (1 - Math.min(1, Math.max(0, Number(intensity)))) * 255;

    return `rgb(${boundedIntensity}, ${boundedIntensity}, ${boundedIntensity})`;
}

function clearPlot(ctx, plotSettings) {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
}

function drawPhenotype(ctx, plotSettings, phenotype) {
    let index = 0;
    let x = 0;
    let y = 0;

    while (index < phenotype.length) {
        ctx.fillStyle = fillStyle(phenotype[index]);
        ctx.fillRect(x + plotSettings.x0, y + plotSettings.y0, plotSettings.cellSize, plotSettings.cellSize);

        index += 1;
        if (index % SEED_SIZE === 0) {
            x = 0;
            y += plotSettings.cellSize + S1;
        } else {
            x += plotSettings.cellSize + S1;
        }
    }
}

export function PhenotypePlot({ phenotype, plotId }) {
    const [plotSettings, setPlotSettings] = useState();

    useEffect(() => {
        if (plotSettings) {
            return;
        }

        const canvas = document.getElementById(plotId);
        const cellSpacing = (SEED_SIZE - 1) * S1;
        const horizontalCellSpace = canvas.width - cellSpacing;
        const verticalCellSpace = canvas.height - cellSpacing;
        const cellMaxWidth = Math.floor(horizontalCellSpace / SEED_SIZE);
        const cellMaxHeight = Math.floor(verticalCellSpace / SEED_SIZE);
        const cellSize = Math.min(cellMaxHeight, cellMaxWidth); 
        const width = cellSpacing + SEED_SIZE * cellSize;
        const height = cellSpacing + SEED_SIZE * cellSize;
        const settings = {
            cellSize,
            x0: Math.floor((canvas.width - width) / 2),
            y0: Math.floor((canvas.height - height) / 2),
            width,
            height,
        }

        console.info("Plot settings:", settings);

        setPlotSettings(settings);
    }, [plotSettings, plotId]);

    useEffect(() => {
        if (!plotSettings) {
            return;
        }

        const canvas = document.getElementById(plotId);
        const ctx = canvas.getContext('2d');

        if (phenotype) {
            drawPhenotype(ctx, plotSettings, phenotype);
        } else {
            clearPlot(ctx, plotSettings);
        }
    }, [plotSettings, plotId, phenotype]);

    return <div className="plot-container">
        <canvas className="plot" id={plotId} height={200}></canvas>
    </div>;
}