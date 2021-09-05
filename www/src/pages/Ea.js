import { useContext } from 'react';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { GenotypePlot } from '../components/GenotypePlot';
import { PhenotypePlot } from '../components/PhenotypePlot';
import { EaSettings, SettingsContext, initialSettings, settingsReducer } from '../components/EaSettings';
import { EaRunner, EaContext } from '../components/EaRunner';
import { EaGraph } from '../components/EaGraph';
import { EaState } from '../components/EaState';

export function Ea() {
    const { ea, eaState, setEaState } = useContext(EaContext);

    return (
        <SettingsContext.Provider value={{ 
            eaSettings: initialSettings,
            eaSettingsDispatch: settingsReducer
        }}>
        <Container>
        <Row>
          <Col lg={4} xs={6}>
              <Container>
                <EaSettings />
                <Row>
                  <Col><EaRunner onStep={setEaState}></EaRunner></Col>
                </Row>
                <Row>
                  <EaState eaState={eaState} />
                </Row>
              </Container>
          </Col>
          <Col lg={8} xs={12}>
            <EaGraph eaState={eaState} />
          </Col>
        </Row>
        <Row>
          <Col lg={3}>
            <h3>Gene distribution</h3>
            <GenotypePlot genotype={eaState?.geneDistribution} />
          </Col>
          <Col lg={3}>
            <h3>Best genotype</h3>
            <GenotypePlot genotype={eaState?.bestGenotype} />
          </Col>
          <Col lg={3}>
            <h3>Cell distribution</h3>
            <PhenotypePlot phenotype={eaState?.cellDistribution} />
          </Col>
          <Col lg={3}>
            <h3>Best phenotype</h3>
            <PhenotypePlot phenotype={eaState?.bestPhenotype} />
          </Col>
        </Row>
      </Container>
    </SettingsContext.Provider>
    )
}