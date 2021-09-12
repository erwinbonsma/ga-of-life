import { useContext } from 'react';

import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { EaControlContext } from './EaControl';

export function EaStatistics() {
    const { eaControl } = useContext(EaControlContext);
    const eaState = eaControl?.eaState;

    if (!eaState) {
        return null;
    }

    const runTimeInSeconds = eaControl.runTime / 1000;

    return (<Container>
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
            <Col className="Label" xs={8}>Avg. fitness</Col>
            <Col className="NumValue" xs={4}>{Math.round(eaState.avgFitness)}</Col>
        </Row>
        <Row>
            <Col className="Label" xs={8}>Run time [s]</Col>
            <Col className="NumValue" xs={4}>{Math.round(runTimeInSeconds)}</Col>
        </Row>
        <Row>
            <Col className="Label" xs={8}>CA steps [s<sup>-1</sup>]</Col>
            <Col className="NumValue" xs={4}>{runTimeInSeconds > 0 ? Math.round(eaState.caSteps / runTimeInSeconds) : "-"}</Col>
        </Row>
    </Container>);
}