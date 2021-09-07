import React, { useContext } from 'react';

import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { GenotypePlot } from '../components/GenotypePlot';
import { PhenotypePlot } from '../components/PhenotypePlot';
import { EaGraph } from '../components/EaGraph';
import { EaStatistics } from '../components/EaStatistics';
import { EaControl, ControlContext } from '../components/EaControl';

export function EaRunner() {
    const { eaControl } = useContext(ControlContext);

    return (<Container>
        <Row>
            <Col lg={4} xs={6}>
                <Container>
                    <Row>
                        <Col><EaControl /></Col>
                    </Row>
                    <Row>
                        <EaStatistics />
                    </Row>
                </Container>
            </Col>
            <Col lg={8} xs={12}>
                <EaGraph />
            </Col>
        </Row>
        <Row>
            <Col lg={3}>
                <h3>Gene distribution</h3>
                <GenotypePlot genotype={eaControl?.eaState?.geneDistribution} />
            </Col>
            <Col lg={3}>
                <h3>Best genotype</h3>
                <GenotypePlot genotype={eaControl?.eaState?.bestGenotype} />
            </Col>
            <Col lg={3}>
                <h3>Cell distribution</h3>
                <PhenotypePlot phenotype={eaControl?.eaState?.cellDistribution} />
            </Col>
            <Col lg={3}>
                <h3>Best phenotype</h3>
                <PhenotypePlot phenotype={eaControl?.eaState?.bestPhenotype} />
            </Col>
        </Row>
    </Container>);
}