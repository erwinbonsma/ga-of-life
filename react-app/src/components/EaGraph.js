import { useEffect, useState } from 'react';

const Highcharts = require('highcharts');
require('highcharts/modules/exporting')(Highcharts);

const historyLen = 1000;

export function EaGraph({ eaState }) {
    const [history, setHistory] = useState([]);
    const [chart, setChart] = useState();

    useEffect(() => {
        if (chart && eaState?.generations !== history[history.length - 1]?.generations) {
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
            chart.series[1].addPoint([eaState.generations, eaState.avgFitness], true, shift);
        }
    }, [eaState, history, chart])

    useEffect(() => {
        setChart(
            Highcharts.chart('ea-graph', {
                series: [
                    {
                        name: 'Max. fitness',
                        type: 'line',
                        data: [],
                    },
                    {
                        name: 'Avg. fitness',
                        type: 'line',
                        data: [],
                    }
                ]
            })
        );
    }, []);

    return (
        <div id="ea-graph" style={{height: "400px", width: "100%" }} ></div>
    )
}