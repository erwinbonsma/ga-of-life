import { useContext, useEffect, useState } from 'react';

import { EaControlContext } from '../components/EaControl';

const Highcharts = require('highcharts');
require('highcharts/modules/exporting')(Highcharts);

const historyLen = 250;

export function EaGraph() {
    const [lastPlotGeneration, setLastPlotGeneration] = useState(0);
    const [numPlotPoints, setNumPlotPoints] = useState(0);
    const [chart, setChart] = useState();

    const { eaControl } = useContext(EaControlContext);
    const eaState = eaControl?.eaState;

    const initChart = function() {
        setChart(
            Highcharts.chart('ea-graph', {
                title: {
                    text: "Evolutionary Algorithm",
                },
                yAxis: [{
                    title: {
                        text: 'Fitness'
                    }
                }, {
                    title: {
                        text: 'Evaluations'
                    },
                    opposite: true
                }],
                series: [
                    {
                        name: 'Max. fitness',
                        type: 'line',
                        marker: { enabled: false },
                        data: [],
                    },
                    {
                        name: 'Num. evaluations',
                        type: 'line',
                        marker: { enabled: false },
                        yAxis: 1,
                        data: [],
                    },
                ]
            })
        );
    };

    useEffect(() => {
        if (!chart) {
            return
        }
        if (!eaState) {
            // EA was reset. Reset graph, if not yet already done.
            if (lastPlotGeneration > 0) {
                chart.destroy();
                initChart();

                setLastPlotGeneration(0);
                setNumPlotPoints(0);
            }
        } else {
            // Add EA state to graph, if not yet already done.
            if (eaState?.generations !== lastPlotGeneration) {
                const shift = numPlotPoints === historyLen;
                chart.series[0].addPoint([eaState.generations, eaState.maxFitness], true, shift);
                chart.series[1].addPoint([eaState.generations, eaState.evaluationsDelta], true, shift);

                setLastPlotGeneration(eaState.generations);
                if (!shift) {
                    setNumPlotPoints(numPlotPoints + 1);
                }
            }
        }
    }, [eaState, chart, numPlotPoints, lastPlotGeneration]);

    useEffect(() => {
        initChart();
    }, []);

    return (
        <div id="ea-graph" style={{height: "400px", width: "100%" }} ></div>
    )
}