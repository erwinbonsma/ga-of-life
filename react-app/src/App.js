import './App.css';
import { useState } from 'react';
import { EaRunner } from './components/EaRunner';

function App() {
  const [eaState, setEaState] = useState();

  return (
    <div className="App">
      <EaRunner onStep={setEaState}></EaRunner>
      { eaState && 
        <p>Generation = {eaState.generations}, Best = {eaState.maxFitness}</p>
      }
    </div>
  );
}

export default App;
