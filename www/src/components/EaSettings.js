import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import React, { useContext } from 'react';

import { EaControlContext } from './EaControl';
import { CaSettingsContext } from './CaControl';
import { bound } from '../shared/utils';

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

    function NumericFormField(
        // Required variables
        id, label, value, actionType,
        // Optional variables
        optionalProps = {}
    ) {
        const {
            disabled = false,
            indent = false,
            dispatch = eaSettingsDispatch,
            stepSize = 1
        } = optionalProps;

        const activeStepSize = (stepSize * 20 <= Math.abs(value)) ? stepSize * 10 : stepSize;

        return (
            <Form.Group as={Row} controlId={id} key={id}>
                { indent && <Col xs={1} /> }
                <Form.Label column xs={indent ? 7 : 8}>{label}</Form.Label>
                <Col xs={4}>
                    <Form.Control type="number" step={activeStepSize} value={value} disabled={disabled}
                        onChange={e => dispatch({ type: actionType, value: e.target.value })} />
                </Col>
            </Form.Group>
        );
    };
    function CheckBoxFormField(
        // Required variables
        id, label, value, actionType,
        // Optional variables
        optionalProps = {}
    ) {
        const {
            dispatch = eaSettingsDispatch
        } = optionalProps;

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
                { NumericFormField(
                    'formGridSize',
                    'Grid size',
                    caSettings.gridSize,
                    'gridSize',
                    {
                        dispatch: caSettingsDispatch,
                        stepSize: 16,
                    }
                )}
                { CheckBoxFormField(
                    'formBorderWraps',
                    'Enable border wrap',
                    caSettings.borderWraps,
                    'borderWraps',
                    {
                        dispatch: caSettingsDispatch
                    }
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
                    'fitnessNumToggledCells'
                )}
                { NumericFormField(
                    'formNumToggledSteps',
                    'Steps to reach',
                    eaSettings.fitnessNumToggledSteps,
                    'fitnessNumToggledSteps',
                    {
                        disabled: !numToggledFitnessEnabled,
                        indent: true
                    }
                )}
                { NumericFormField(
                    'formMaxAliveCells',
                    'Maximum alive cells',
                    eaSettings.fitnessMaxAliveCells,
                    'fitnessMaxAliveCells'
                )}
                { NumericFormField(
                    'formMaxAliveSteps',
                    'Step to reach',
                    maxAliveFitnessEnabled ? eaSettings.fitnessMaxAliveSteps : 0,
                    'fitnessMaxAliveSteps',
                    {
                        disabled: !maxAliveFitnessEnabled,
                        indent: true
                    }
                )}
                { NumericFormField(
                    'formNumStartCells',
                    'Number of cells at start',
                    eaSettings.fitnessNumStartCells,
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
                    'populationSize',
                    {
                        stepSize: 10
                    }
                )}
                { NumericFormField(
                    'formRecombinationRate',
                    'Recombination rate',
                    eaSettings.recombinationRate,
                    'recombinationRate',
                    {
                        stepSize: 0.1
                    }
                )}
                { NumericFormField(
                    'formMutationRate',
                    'Mutation rate',
                    eaSettings.mutationRate,
                    'mutationRate',
                    {
                        stepSize: 0.1
                    }
                )}
                { NumericFormField(
                    'formTournamentSize',
                    'Tournament size',
                    eaSettings.tournamentSize,
                    'tournamentSize'
                )}
                { CheckBoxFormField(
                    'formElitism',
                    'Use elitism',
                    eaSettings.elitism,
                    'elitism'
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