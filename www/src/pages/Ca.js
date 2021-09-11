import { useReducer } from 'react';

import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { useHistory } from "react-router-dom";

import { CaControl, CaControlContext, caControlReducer, initialCaControlState } from '../components/CaControl';
import { CaGraph } from '../components/CaGraph';

export function Ca({ seed }) {
    const history = useHistory();
    const [caControl, caControlDispatch] = useReducer(caControlReducer, initialCaControlState);

    if (!seed) {
        console.info("No seed. Redirecting to main page");
        history.push("/");
    }

    return (seed && <Container>
        <Row>
            <CaControlContext.Provider value={{ caControl, caControlDispatch }}>
                <Col lg={4} xs={6}>
                    <CaControl seed={seed} />
                </Col>
                <Col lg={8} xs={12}>
                    <CaGraph />
                </Col>
            </CaControlContext.Provider>
        </Row>
    </Container>);
}