import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { EaSettings } from '../components/EaSettings';

export function EaSetup() {
    return (<Container>
        <Row>
            <Col xs={0} md={1} lg={2} xl={3}/>
            <Col xs={12} md={10} lg={8} xl={6}><EaSettings /></Col>
            <Col xs={0} md={1} lg={2} xl={3}/>
        </Row>
    </Container>)
}