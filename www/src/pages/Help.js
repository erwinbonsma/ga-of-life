
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';

export function Help() {
    return (<Container>
        <Row className="mt-3">
            <p>
                This application lets you use a Genetic Algorithm to solve an optimization problems in Conway's Game of Life.
            </p>
        </Row>
        <Row as="h2">
            Conway's Game of Life
        </Row>
        <Row>
            <p>
                <a href="https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life">Conway's Game of Life</a> is a Cellular Automaton (CA) devised by John Conway.
                It consists of a simple set of rules that specify how a set of cells in a two-dimensional grid behave.
                Each cell can only have two states: dead or alive.
                The state of a cell in the next generation of the CA depends on its current state, as well as those of its eight neighbours.
            </p>
            <p>
                The rules are as follows:
                <ul>
                    <li>An alive cell with two or three alive neighbours remains alive</li>
                    <li>A dead cell with exactly three alive neighbours becomes alive</li>
                    <li>The state of all other cells remains unchanged</li>
                </ul>
            </p>
        </Row>
        <Row as="h2">
            The Optimization Problem
        </Row>
        <Row>
            <p>
                You can solve variants of the following problem:
                    Starting with an 8x8 group of cells at the center of the CA grid, what is the configuration
                    that covers most of the grid?
            </p>
            <p>
                More specifically, the following aspects can be rewarded:
                <ul>
                    <li><em>Number of toggled cells</em>: The number of cells that were alive at least once</li>
                    <li><em>Maximum alive cells</em>: The maximum number of cells that were alive at a given moment</li>
                    <li><em>The number of cells at start</em>: The number of alive cells at the start</li>
                </ul>
                By specifying weights you control which aspects are considered.
                Aspects are ignored when the weight is zero.
                Via the sign of the weights you can choose if the objective is to maximise or minimize the aspect.
                Typically, the first two aspects are maximised and the last one minimized, but do what you like.
            </p>
        </Row>
        <Row as="h2">
           Genetic Algorithm
        </Row>
        <Row>
            <p>
                A <em>Genetic Algorithm</em> (GA) is a nature-inspired optimization technique that mimics natural selection.
            </p>
            <p>
                A GA uses a <em>population</em> of <em>individuals</em>.
                Each individual has a <em>genotype</em>, in this case a string of bits.
                Each genotype encodes a <em>phenotype</em>, a possible solution to the problem.
                For this problem a neutral encoding is used, which means that many genotypes translate to the the same phenotype.
                This type of encoding can help to prevent premature convergence of the optimization algorithm.
                It can reduce the probability that the algorithm gets stuck in a poor quality sub-optimal solution.
            </p>
            <p>
                For each individual the GA determines a <em>fitness</em>.
                For this problem, the GA does so as follows:
                <ol>
                    <li>It creates a CA with a starting configuration as specified by the individual's phenotype</li>
                    <li>It executes the CA until it seems to have stabilized (according to some heuristic rules)</li>
                    <li>It calculates a fitness by scoring the aspects according to the configured weights</li>
                </ol>
            </p>
            <p>
                The fitness is used to select individuals on which to base the next generation of the population.
                Individuals with a higher fitness are more likely to be selected.
                Individuals can reproduce in two different ways:
                <ul>
                    <li>
                        An individual can reproduce sexually.
                        In this case, two individuals are selected, and their genotypes are <em>recombined</em> to form a new individual.
                    </li>
                    <li>
                        Alternatively, an individual reproduces a-sexually.
                        It then reproduces by cloning its genotype.
                    </li>
                </ul>
                In both cases, the genotype of the new individual is typically <em>mutated</em> to introduce variation.
            </p>
            <p>
                Selection optionally uses <em>elitism</em>.
                When enabled, the best individual is always selected and copied without mutation to the new generation.
                This ensures that the best solution is never forgotten.
                A drawback is that it can lead to premature convergence.
                Elitism reduces the genetic diversity, which may limit the quality of the solution that is found.
            </p>
        </Row>
    </Container>);
}