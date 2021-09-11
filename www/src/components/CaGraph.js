import { useContext, useEffect, useState } from 'react';

import { CaControlContext } from '../components/CaControl';

const Highcharts = require('highcharts');
require('highcharts/modules/exporting')(Highcharts);

export function CaGraph() {
    const [numStepsPlotted, setNumStepsPlotted] = useState(0);
    const [chart, setChart] = useState();

    const { caControl } = useContext(CaControlContext);
    const caState = caControl?.caStats;

    const initChart = function() {
        setChart(
            Highcharts.chart('ea-graph', {
                title: {
                    text: "CA Stats",
                },
                yAxis: [{
                    floor: 0,
                    title: {
                        text: 'Num. alive cells'
                    }
                }, {
                    floor: 0,
                    title: {
                        text: 'Num. once alive cells'
                    },
                    opposite: true,
                }],
                series: [
                    {
                        name: 'Num. alive',
                        type: 'line',
                        marker: { enabled: false },
                        data: [],
                    },
                    {
                        name: 'Num. once alive',
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
        if (!caState) {
            if (numStepsPlotted > 0) {
                chart.destroy();
                initChart();

                setNumStepsPlotted(0);                
            }
        } else {
            if (caControl.numSteps % 10 === 0 && caControl.numSteps !== numStepsPlotted ) {
                chart.series[0].addPoint([caControl.numSteps, caState.numAlive], true, false);
                chart.series[1].addPoint([caControl.numSteps, caState.numOnceAlive], true, false);

                setNumStepsPlotted(caControl.numSteps);
            }
        }
    }, [caState, caControl?.numSteps, numStepsPlotted, chart])

    useEffect(() => {
        initChart();
    }, []);

    return (
        <div id="ea-graph" style={{height: "400px", width: "100%" }} ></div>
    )
}