import './App.css';
import { useReducer } from 'react';
import { HashRouter, Link, Route, Switch } from 'react-router-dom';

import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';

import { CaSettingsContext, caSettingsReducer, initialCaSettings } from './components/CaControl';
import { EaControlContext, eaControlReducer, initialEaControlState } from './components/EaControl';
import { Ca } from './pages/Ca'
import { Ea } from './pages/Ea'

function App() {
    const [caSettings, caSettingsDispatch] = useReducer(caSettingsReducer, initialCaSettings);
    const [eaControl, eaControlDispatch] = useReducer(eaControlReducer, initialEaControlState);

    return (
        <div className="App">
            <Navbar bg="primary" variant="dark">
                <Container>
                    <Navbar.Brand href="#/" >Evolving Live</Navbar.Brand>
                    <Nav defaultActiveKey="#/">
                        <Nav.Link href="#/" >EA</Nav.Link>
                        <Nav.Link href="#/ca" >CA</Nav.Link>
                    </Nav>
                </Container>
            </Navbar>
            <EaControlContext.Provider value={{ eaControl, eaControlDispatch }}>
                <CaSettingsContext.Provider value={{ caSettings, caSettingsDispatch }}>
                    <HashRouter basename="/">
                        <Switch>
                            <Route exact path="/">
                                <Ea/>
                            </Route>
                            <Route exact path="/ca">
                                <Ca seed={eaControl?.eaState?.bestPhenotype} />
                            </Route>
                            <Route path="*">
                                <p>Page not found</p>
                                <Link to="/">Go back</Link>
                            </Route>
                        </Switch>
                    </HashRouter>
                </CaSettingsContext.Provider>
            </EaControlContext.Provider>
        </div>
    );
}

export default App;
