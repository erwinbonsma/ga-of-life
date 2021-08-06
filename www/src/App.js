import './App.css';
import { useState } from 'react';
import { EaGraph } from './components/EaGraph';
import { EaRunner } from './components/EaRunner';
import { CaRunner } from './components/CaRunner';

function App() {
  const [eaState, setEaState] = useState();

  return (
    <div className="App">
      <EaRunner onStep={setEaState}></EaRunner>
      { eaState && <>
          <p>Generation = {eaState.generations}, 
            #Evaluations = {eaState.evaluations},
            #CA steps = {eaState.caSteps},
            Best = {eaState.maxFitness}</p>
          <pre>{eaState.bestPhenotype}</pre>
        </>
      }
      <EaGraph eaState={eaState} />
      <CaRunner seed={eaState?.bestPhenotype} />
    </div>
  );
}

export default App;
