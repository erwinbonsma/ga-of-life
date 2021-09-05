import './App.css';
import { useReducer } from 'react';
import { HashRouter, Link, Route, Switch } from 'react-router-dom';

import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';

import { ControlContext, eaControlReducer } from './components/EaControl';
import { Ca } from './pages/Ca'
import { Ea } from './pages/Ea'

function App() {
    const [eaControl, eaControlDispatch] = useReducer(eaControlReducer);

    return (
        <div className="App">
            <Navbar bg="primary" variant="dark">
                <Container>
                    <Navbar.Brand href="#/" >Evolving Live</Navbar.Brand>
                    <Nav>
                        <Nav.Link href="#/" >EA</Nav.Link>
                        <Nav.Link href="#/ca" >CA</Nav.Link>
                    </Nav>
                </Container>
            </Navbar>
            <ControlContext.Provider value={{ eaControl, eaControlDispatch }}>
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
            </ControlContext.Provider>
        </div>
    );
}

export default App;
