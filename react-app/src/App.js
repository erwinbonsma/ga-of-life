import './App.css';
import { useState } from 'react';
import { EaGraph } from './components/EaGraph';
import { EaRunner } from './components/EaRunner';

function App() {
  const [eaState, setEaState] = useState();

  return (
    <div className="App">
      <EaRunner onStep={setEaState}></EaRunner>
      { eaState && <>
          <p>Generation = {eaState.generations}, Best = {eaState.maxFitness}</p>
          <pre>{eaState.bestPhenotype}</pre>
        </>
      }
      <EaGraph eaState={eaState} />
    </div>
  );
}

export default App;
