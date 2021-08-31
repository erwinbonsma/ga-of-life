import './App.css';
import { useState, useReducer } from 'react';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { EaGraph } from './components/EaGraph';
import { EaRunner } from './components/EaRunner';
import { EaSettings, SettingsContext, initialSettings, settingsReducer } from './components/EaSettings';
import { EaState } from './components/EaState';
import { CaRunner } from './components/CaRunner';
import { GenotypePlot } from './components/GenotypePlot';
import { PhenotypePlot } from './components/PhenotypePlot';

function App() {
  const [eaState, setEaState] = useState();
  const [eaSettings, eaSettingsDispatch] = useReducer(settingsReducer, initialSettings);

  return (
    <div className="App">

      <Container>
        <Row>
          <Col lg={4} xs={6}>
            <SettingsContext.Provider value={{
              settings: eaSettings,
              dispatch: eaSettingsDispatch
            }}>
              <EaSettings />
              <Container>
                <Row>
                  <Col><EaRunner onStep={setEaState}></EaRunner></Col>
                </Row>
                <Row>
                  <EaState eaState={eaState} />
                </Row>
              </Container>
            </SettingsContext.Provider>
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
      <CaRunner seed={eaState?.bestPhenotype} />
    </div>
  );
}

export default App;
