import { useContext } from 'react';

import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { ControlContext } from '../components/EaControl';

export function EaState() {
    const { eaControl } = useContext(ControlContext);
    const eaState = eaControl?.eaState;

    return (eaState && (
        <Container>
            <Row>
                <Col className="Label" xs={8}>Num. generations</Col>
                <Col className="NumValue" xs={4}>{eaState.generations}</Col>
            </Row>
            <Row>
                <Col className="Label" xs={8}>Num. evaluations</Col>
                <Col className="NumValue" xs={4}>{eaState.evaluations}</Col>
            </Row>
            <Row>
                <Col className="Label" xs={8}>Num. CA steps</Col>
                <Col className="NumValue" xs={4}>{eaState.caSteps}</Col>
            </Row>
            <Row>
                <Col className="Label" xs={8}>Max. fitness</Col>
                <Col className="NumValue" xs={4}>{eaState.maxFitness}</Col>
            </Row>
            <Row>
                <Col className="Label" xs={8}>Run time [s]</Col>
                <Col className="NumValue" xs={4}>{eaControl.runTime}</Col>
            </Row>
            <Row>
                <Col className="Label" xs={8}>CA steps [s<sup>-1</sup>]</Col>
                <Col className="NumValue" xs={4}>{eaControl.runTime > 0 ? Math.round(eaState.caSteps / eaControl.runTime) : "-"}</Col>
            </Row>
        </Container>
    )) || null;
}