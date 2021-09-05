import Form from 'react-bootstrap/Form';
import Col from 'react-bootstrap/Col';
import Row from 'react-bootstrap/Row';
import React, { useContext } from 'react';

export const SettingsContext = React.createContext();

function bound(value, min, max) {
    return Math.max(Math.min(max, value), min);
}

export function settingsReducer(state, action) {
    console.log("dispatched", state, action);
    switch (action.type) {
        case 'populationSize': return {
            ...state, populationSize: bound(action.value, 10, 1000)
        };
        case 'recombinationRate': return {
            ...state, recombinationRate: bound(action.value, 0, 1)
        };
        case 'mutationRate': return {
            ...state, mutationRate: bound(action.value, 0, 1)
        };
        case 'tournamentSize': return {
            ...state, tournamentSize: bound(action.value, 1, 5)
        };
        case 'elitism': return {
            ...state, elitism: action.value
        }
        default:
            console.error('Unexpected action:', action.type);
    }
}

export const initialSettings = {
    populationSize: 100,
    recombinationRate: 0.4,
    mutationRate: 0.9,
    tournamentSize: 2,
    elitism: true,
};

export function EaSettings() {
    const { eaSettings, eaSettingsDispatch } = useContext(SettingsContext);

    return <Form>
        <Form.Group as={Row} controlId="formPopulationSize">
            <Form.Label column sm={6}>Population size</Form.Label>
            <Col sm={6}>
                <Form.Control type="number" step="10"
                    value={eaSettings.populationSize}
                    onChange={e => eaSettingsDispatch({ type: 'populationSize', value: e.target.value })} />
            </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="formRecombinationRate">
            <Form.Label column sm={6}>Recombination rate</Form.Label>
            <Col sm={6}>
                <Form.Control type="number" step="0.1"
                    value={eaSettings.recombinationRate}
                    onChange={e => eaSettingsDispatch({ type: 'recombinationRate', value: e.target.value })} />
            </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="formMutationRate">
            <Form.Label column sm={6}>Mutation rate</Form.Label>
            <Col sm={6}>
                <Form.Control type="number" step="0.1"
                    value={eaSettings.mutationRate}
                    onChange={e => eaSettingsDispatch({ type: 'mutationRate', value: e.target.value })} />
            </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="formTournamentSize">
            <Form.Label column sm={6}>Tournament size</Form.Label>
            <Col sm={6}>
                <Form.Control type="number" step="1"
                    value={eaSettings.tournamentSize}
                    onChange={e => eaSettingsDispatch({ type: 'tournamentSize', value: e.target.value })} />
            </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="formElitism">
            <Form.Label column sm={6}>Use elitism</Form.Label>
            <Col sm={6}>
                <Form.Check type="checkbox" 
                    checked={eaSettings.elitism}
                    onChange={e => eaSettingsDispatch({ type: 'elitism', value: e.target.checked })} />
            </Col>
        </Form.Group>
    </Form>
}