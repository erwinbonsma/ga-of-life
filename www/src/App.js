import './App.css';
import { useState } from 'react';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { EaGraph } from './components/EaGraph';
import { EaRunner } from './components/EaRunner';
import { CaRunner } from './components/CaRunner';
import { GenotypePlot } from './components/GenotypePlot';

function App() {
  const [eaState, setEaState] = useState();

  return (
    <div className="App">
      <EaRunner onStep={setEaState}></EaRunner>

      <Container>
        <Row>
          <Col xs={4}>
            { eaState && <>
              <p>Generation = {eaState.generations}, 
              #Evaluations = {eaState.evaluations},
              #CA steps = {eaState.caSteps},
              Best = {eaState.maxFitness}</p>
              <pre>{eaState.bestPhenotype}</pre>
            </>}
          </Col>
          <Col xs={4}>
            <GenotypePlot genotype={eaState?.bestGenotype} plotId="best-genotype" />
          </Col>
          <Col xs={4}>
            <GenotypePlot genotype={eaState?.geneDistribution} plotId="gene-distribution" />
          </Col>
        </Row>
      </Container>
      <EaGraph eaState={eaState} />
      <CaRunner seed={eaState?.bestPhenotype} />
    </div>
  );
}

export default App;
