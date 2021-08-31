import Form from 'react-bootstrap/Form';
import Col from 'react-bootstrap/Col';
import Row from 'react-bootstrap/Row';
import React, { useContext } from 'react';

export const SettingsContext = React.createContext();

export function settingsReducer(state, action) {
    console.log("dispatched", state, action);
    switch (action.type) {
        case 'populationSize': return {
            ...state, populationSize: action.value
        };
        default:
            console.error('Unexpected action:', action.type);
    }
}

export const initialSettings = {
    populationSize: 120
};
    
export function EaSettings() {
    const { settings, dispatch } = useContext(SettingsContext);

    return <Form>
        <Form.Group as={Row} controlId="formPopulationSize">
            <Form.Label column sm={6}>Population size</Form.Label>
            <Col sm={6}>
                <Form.Control type="number" step="10" min="10" max="1000" value={settings.populationSize}
                    onChange={e => dispatch({ type: 'populationSize', value: e.target.value })}
                />
            </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="formRecombinationRate">
            <Form.Label column sm={6}>Recombination rate</Form.Label>
            <Col sm={6}>
                <Form.Control type="number" step="0.1" min="0" max="1" />
            </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="formMutationRate">
            <Form.Label column sm={6}>Mutation rate</Form.Label>
            <Col sm={6}>
                <Form.Control type="number" step="0.1" min="0" max="1" />
            </Col>
        </Form.Group>
    </Form>
}