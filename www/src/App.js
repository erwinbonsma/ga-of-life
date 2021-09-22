import './App.css';
import { useReducer } from 'react';
import { HashRouter, Link, Route, Switch } from 'react-router-dom';

import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';

import { CaSettingsContext, caSettingsReducer, initialCaSettings } from './components/CaControl';
import { EaControlContext, eaControlReducer, initialEaControlState } from './components/EaControl';
import { EaSettingsContext, initialEaSettings, eaSettingsReducer } from './components/EaSettings';
import { Ca } from './pages/Ca'
import { Ea } from './pages/Ea'
import { Help } from './pages/Help'

function App() {
    const [caSettings, caSettingsDispatch] = useReducer(caSettingsReducer, initialCaSettings);
    const [eaControl, eaControlDispatch] = useReducer(eaControlReducer, initialEaControlState);
    const [eaSettings, eaSettingsDispatch] = useReducer(eaSettingsReducer, initialEaSettings);

    return (
        <div className="App">
            <Navbar bg="primary" variant="dark">
                <Container>
                    <Navbar.Brand href="#/" >Evolving Live</Navbar.Brand>
                    <Nav defaultActiveKey="#/">
                        <Nav.Link href="#/" >GA</Nav.Link>
                        <Nav.Link href="#/ca" >CA</Nav.Link>
                        <Nav.Link href="#/help" >Help</Nav.Link>
                    </Nav>
                </Container>
            </Navbar>
            <EaControlContext.Provider value={{ eaControl, eaControlDispatch }}>
                <EaSettingsContext.Provider value={{ eaSettings, eaSettingsDispatch }}>
                    <CaSettingsContext.Provider value={{ caSettings, caSettingsDispatch }}>
                        <HashRouter basename="/">
                            <Switch>
                                <Route exact path="/">
                                    <Ea/>
                                </Route>
                                <Route exact path="/ca">
                                    <Ca seed={eaControl?.eaState?.bestPhenotype} />
                                </Route>
                                <Route exact path="/help">
                                    <Help />
                                </Route>
                                <Route path="*">
                                    <p>Page not found</p>
                                    <Link to="/">Go back</Link>
                                </Route>
                            </Switch>
                        </HashRouter>
                    </CaSettingsContext.Provider>
                </EaSettingsContext.Provider>
            </EaControlContext.Provider>
        </div>
    );
}

export default App;
