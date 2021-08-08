import './App.css';
import { useState } from 'react';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

import { EaGraph } from './components/EaGraph';
import { EaRunner } from './components/EaRunner';
import { CaRunner } from './components/CaRunner';
import { GenotypePlot } from './components/GenotypePlot';
import { PhenotypePlot } from './components/PhenotypePlot';

function App() {
  const [eaState, setEaState] = useState();

  return (
    <div className="App">

      <Container>
        <Row>
          <Col lg={4} xs={6}>
            <Container>
              <Row>
                <Col><EaRunner onStep={setEaState}></EaRunner></Col>
              </Row>
              <Row>
                <Col>
                  { eaState && <p>
                    Generation = {eaState.generations}<br></br>
                    #Evaluations = {eaState.evaluations}<br></br>
                    #CA steps = {eaState.caSteps}<br></br>
                    Best = {eaState.maxFitness}</p>}
                </Col>
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
            <GenotypePlot genotype={eaState?.geneDistribution} plotId="gene-distribution" />
          </Col>
          <Col lg={3}>
            <h3>Best genotype</h3>
            <GenotypePlot genotype={eaState?.bestGenotype} plotId="best-genotype" />
          </Col>
          <Col lg={3}>
            <h3>Cell distribution</h3>
            <PhenotypePlot phenotype={eaState?.cellDistribution} plotId="cell-distribution" />
          </Col>
          <Col lg={3}>
            <h3>Best phenotype</h3>
            <PhenotypePlot phenotype={eaState?.bestPhenotype} plotId="best-phenotype" />
          </Col>
        </Row>
      </Container>
      <CaRunner seed={eaState?.bestPhenotype} />
    </div>
  );
}

export default App;
