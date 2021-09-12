import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import Col from 'react-bootstrap/Col';
import Row from 'react-bootstrap/Row';
import React, { useContext } from 'react';

import { EaControlContext } from './EaControl';
import { CaSettingsContext } from './CaControl';

export const EaSettingsContext = React.createContext();

export const initialEaSettings = {
    populationSize: 100,
    recombinationRate: 0.4,
    mutationRate: 0.9,
    tournamentSize: 2,
    elitism: true,
};

function bound(value, min, max) {
    return Math.max(Math.min(max, value), min);
}

export function eaSettingsReducer(state, action) {
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
        };
        default:
            console.error('Unexpected action:', action.type);
    }
}

export function EaSettings() {
    const { caSettings, caSettingsDispatch } = useContext(CaSettingsContext);
    const { eaSettings, eaSettingsDispatch } = useContext(EaSettingsContext);
    const { eaControlDispatch } = useContext(EaControlContext);

    return <Form>
        <h1>Problem Settings</h1>
        <Form.Group as={Row} controlId="formBorderWraps">
            <Form.Label column sm={6}>Border wraps</Form.Label>
            <Col sm={6}>
                <Form.Check type="checkbox" 
                    checked={caSettings.borderWraps}
                    onChange={e => caSettingsDispatch({ type: 'borderWraps', value: e.target.checked })} />
            </Col>
        </Form.Group>
        <h1>EA Settings</h1>
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
        <Button variant="primary" type="submit"
            onClick={() => eaControlDispatch({ type: 'initialize', settings: { ...eaSettings, ...caSettings }})} >
            Start
        </Button>
    </Form>
}