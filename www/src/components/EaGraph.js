import { useContext, useEffect, useState } from 'react';

import { ControlContext } from '../components/EaControl';

const Highcharts = require('highcharts');
require('highcharts/modules/exporting')(Highcharts);

const historyLen = 250;

export function EaGraph() {
    const [history, setHistory] = useState([]);
    const [chart, setChart] = useState();

    const { eaControl } = useContext(ControlContext);
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
            if (history.length > 0) {
                setHistory([]);

                chart.destroy();
                initChart();
            }
        } else {
            // Add EA state to graph, if not yet already done.
            if (eaState?.generations !== history[history.length - 1]?.generations) {
                var shift = false;
                if (history.length < historyLen) {
                    setHistory([
                        ...history, eaState
                    ]);
                } else {
                    setHistory([
                        ...history.slice(1, historyLen), eaState
                    ]);
                    shift = true;
                }
                chart.series[0].addPoint([eaState.generations, eaState.maxFitness], true, shift);
                chart.series[1].addPoint([eaState.generations, eaState.evaluationsDelta], true, shift);
            }
        }
    }, [eaState, history, chart])

    useEffect(() => {
        initChart();
    }, []);

    return (
        <div id="ea-graph" style={{height: "400px", width: "100%" }} ></div>
    )
}