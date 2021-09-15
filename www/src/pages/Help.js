
import Container from 'react-bootstrap/Container';

export function Help() {
    return (<Container>
        <p>
            This application lets you use an Evolutionary Algorithm to solve optimization problems in
            Conway's Game of Life.
        </p>
        <h2>Conway's Game of Life</h2>
        <p>
            <a href="https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life">Conway's Game of Life</a> is a Cellular Automaton (CA) devised by John Conway.
            It consists of a simple set of rules that specify how a set of cells in a two-dimensional
            grid behave.
            Each cell can only have two states: dead or alive.
            The state of a cell in the next generation of the CA depends on its state, as well as those
            of its eight neighbours.
        </p>
        <p>
            The rules are as follows:
            <ul>
                <li>An alive cell with two or three alive neighbours remains alive</li>
                <li>A dead cell with exactly three alive neighbours becomes alive</li>
                <li>The state of all other cells remains unchanged</li>
            </ul>
        </p>
        <h2>The Optimization Problem</h2>
        <p>
            You can solve variants of the following problem:
                Starting with an 8x8 group of cells at the center of the CA grid, what is the configuration
                that covers most of the grid?
        </p>
        <p>
            More specifically, the following aspects can be rewarded:
            <ul>
                <li><b>Number of toggled cells</b>: The number of cells that were at least briefly alive</li>
                <li><b>Maximum alive cells</b>: The maximum number of cells that were alive at a given moment</li>
                <li><b>The number of cells at start</b>: The number of alive cells at the start</li>
            </ul>
            By specifying weights you control which aspects are considered.
            Aspects are ignored when the weight is zero.
            Via the sign of the weights you can choose if the objective is to maximise or minimize the aspect.
            Typically, the first two aspects are maximised, and the last one minimized, but do what you like.
        </p>
        <h2>Evolutionary Algorithm</h2>
        <p>
            An Evolutionary Algorithm (EA) is a nature-inspired optimization technique.
            It 
        </p>
        <p>
            The EA uses a <em>population</em> of <em>individuals</em>.
            Each individual has a <em>genotype</em>, in this case a string of bits.
            Each genotype encodes a <em>phenotype</em>, a possible solution to the problem.
            For this problem a neutral encoding is used, which means that many genotypes translate to the the
            same phenotype.
            This type of encoding can help to prevent premature convergence of the optimization algorithm and
            can reduce the probability that the algorithm gets stuck in a poor quality sub-optimal solution.
        </p>
        <p>
            For each individual the EA determines a <em>fitness</em>.
            For this problem, it does this as follows:
            <ol>
                <li>It creates a CA with a starting configuration as specified by its phenotype</li>
                <li>It executes the CA until its does not seem to improve anymore (according to a heuristic)</li>
                <li>It calculates a fitness by scoring the aspects according to the weights specified in the settings</li>
            </ol>
        </p>
        <p>
            The fitness is used to select individuals on which to base the next generation of the population.
            Individuals with a higher fitness are more likely to be selected.
            Individuals can reproduce in two different ways.
            They can reproduce sexually.
            In this case, two individuals are selected, and their genotypes are <em>recombined</em> to form a new individual.
            Alternatively, an individual reproduces by cloning its genotype.
            In both cases, the genotype of the new individual is typically <em>mutated</em> to introduce variation and possibly
            explore new solutions.
        </p>
        <p>
            Optionally selection uses <em>elitism</em>.
            When enabled, the best individual is always selected and copied without mutation to the new generation.
            This ensures that the best solution is never forgotten.
            However, the drawback is that it may lead to premature convergence.
            Elitism may reduce the genetic diversity too quickly, which may limit the quality of the solution that is found.
        </p>
    </Container>);
}