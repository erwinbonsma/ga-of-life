import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
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
    fitnessNumToggledCells: 1.0,
    fitnessNumToggledSteps: 0.0,
    fitnessMaxAliveCells: 0.0,
    fitnessMaxAliveSteps: 0.0,
    fitnessNumStartCells: 0.0,
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
        case 'fitnessNumToggledCells': return {
            ...state, fitnessNumToggledCells: bound(action.value, -100, 100)
        };
        case 'fitnessNumToggledSteps': return {
            ...state, fitnessNumToggledSteps: bound(action.value, -100, 100)
        };
        case 'fitnessMaxAliveCells': return {
            ...state, fitnessMaxAliveCells: bound(action.value, -100, 100)
        };
        case 'fitnessMaxAliveSteps': return {
            ...state, fitnessMaxAliveSteps: bound(action.value, -100, 100)
        };
        case 'fitnessNumStartCells': return {
            ...state, fitnessNumStartCells: bound(action.value, -100, 100)
        };
        default:
            console.error('Unexpected action:', action.type);
    }
}

export function EaSettings() {
    const { caSettings, caSettingsDispatch } = useContext(CaSettingsContext);
    const { eaSettings, eaSettingsDispatch } = useContext(EaSettingsContext);
    const { eaControlDispatch } = useContext(EaControlContext);

    function NumericFormField(id, label, value, stepSize, actionType, disabled, indent) {
        return (
            <Form.Group as={Row} controlId={id} key={id}>
                { indent && <Col xs={1} /> }
                <Form.Label column xs={indent ? 7 : 8}>{label}</Form.Label>
                <Col xs={4}>
                    <Form.Control type="number" step={stepSize} value={value} disabled={disabled}
                        onChange={e => eaSettingsDispatch({ type: actionType, value: e.target.value })} />
                </Col>
            </Form.Group>
        );
    };
    function CheckBoxFormField(id, label, value, actionType, dispatch) {
        return (
            <Form.Group as={Row} controlId={id}>
                <Form.Label column xs={8}>{label}</Form.Label>
                <Col xs={4}>
                    <Form.Check type="checkbox" 
                        checked={value}
                        onChange={e => dispatch({ type: actionType, value: e.target.checked })} />
                </Col>
            </Form.Group>
        );
    };

    const maxAliveFitnessEnabled = Math.abs(eaSettings.fitnessMaxAliveCells) >= 0.01;
    const numToggledFitnessEnabled = Math.abs(eaSettings.fitnessNumToggledCells) >= 0.01;
    
    return <Form as={Container}>
        <Row>
            <Col><h1>Settings</h1></Col>
        </Row>
        <Row className="SettingsRow pt-2 mt-2 mb-2">
            <Col sm={12} md={3}><h5>CA</h5></Col>
            <Col sm={12} md={9}>
                { CheckBoxFormField(
                    'formBorderWraps',
                    'Enable border wrap',
                    caSettings.borderWraps,
                    'borderWraps',
                    caSettingsDispatch
                )}
            </Col>
        </Row>
        <Row className="SettingsRow pt-2 mt-2 mb-2">
            <Col sm={12} md={3}><h5>Fitness</h5></Col>
            <Col sm={12} md={9}>
                { NumericFormField(
                    'formNumToggledCells',
                    'Number of toggled cells',
                    eaSettings.fitnessNumToggledCells,
                    1,
                    'fitnessNumToggledCells'
                )}
                { NumericFormField(
                    'formNumToggledSteps',
                    'Steps to reach',
                    eaSettings.fitnessNumToggledSteps,
                    1,
                    'fitnessNumToggledSteps',
                    !numToggledFitnessEnabled,
                    true
                )}
                { NumericFormField(
                    'formMaxAliveCells',
                    'Maximum alive cells',
                    eaSettings.fitnessMaxAliveCells,
                    1,
                    'fitnessMaxAliveCells'
                )}
                { NumericFormField(
                    'formMaxAliveSteps',
                    'Step to reach',
                    maxAliveFitnessEnabled ? eaSettings.fitnessMaxAliveSteps : 0,
                    1,
                    'fitnessMaxAliveSteps',
                    !maxAliveFitnessEnabled,
                    true
                )}
                { NumericFormField(
                    'formNumStartCells',
                    'Number of cells at start',
                    eaSettings.fitnessNumStartCells,
                    1,
                    'fitnessNumStartCells'
                )}
            </Col>
        </Row>
        <Row className="SettingsRow pt-2 mt-2 mb-2">
            <Col sm={12} md={3}><h5>Solver</h5></Col>
            <Col sm={12} md={9}>
                { NumericFormField(
                    'formPopulationSize',
                    'Population size',
                    eaSettings.populationSize,
                    10,
                    'populationSize'
                )}
                { NumericFormField(
                    'formRecombinationRate',
                    'Recombination rate',
                    eaSettings.recombinationRate,
                    0.1,
                    'recombinationRate'
                )}
                { NumericFormField(
                    'formMutationRate',
                    'Mutation rate',
                    eaSettings.mutationRate,
                    0.1,
                    'mutationRate'
                )}
                { NumericFormField(
                    'formTournamentSize',
                    'Tournament size',
                    eaSettings.tournamentSize,
                    1,
                    'tournamentSize'
                )}
                { CheckBoxFormField(
                    'formElitism',
                    'Use elitism',
                    eaSettings.elitism,
                    'elitism',
                    eaSettingsDispatch
                )}
            </Col>
        </Row>
        <Row>
            <Col><center>
            <Button variant="primary" type="submit"
                onClick={() => eaControlDispatch({ type: 'initialize', settings: { ...eaSettings, ...caSettings }})} >
                Start
            </Button>
            </center></Col>
        </Row>
    </Form>
}