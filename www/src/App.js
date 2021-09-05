import './App.css';
import { useState } from 'react';
import { HashRouter, Link, Route, Switch } from 'react-router-dom';

import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';

import { EaContext } from './components/EaRunner';
import { Ca } from './pages/Ca'
import { Ea } from './pages/Ea'

function App() {
    const [ea, setEa] = useState();
    const [eaState, setEaState] = useState();

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
            <EaContext.Provider value={{ ea, setEa, eaState, setEaState }}>
                <HashRouter basename="/">
                    <Switch>
                        <Route exact path="/">
                            <Ea/>
                        </Route>
                        <Route exact path="/ca">
                            <Ca seed={eaState?.bestPhenotype} />
                        </Route>
                        <Route path="*">
                            <p>Page not found</p>
                            <Link to="/">Go back</Link>
                        </Route>
                    </Switch>
                </HashRouter>
            </EaContext.Provider>
        </div>
    );
}

export default App;
