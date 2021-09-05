import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { EaSettings } from '../components/EaSettings';

export function EaSetup() {
    return (<Container>
        <Row>
            <Col lg="3" />
            <Col lg="6"><EaSettings /></Col>
            <Col lg="3" />
        </Row>
    </Container>)
}