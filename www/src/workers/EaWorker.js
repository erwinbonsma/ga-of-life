let ea;
export async function init() {
    if (!ea) {
        const wasm = await import('ga-of-life');

        ea = new wasm.MyEvolutionaryAlgorithm();
    }

    return ea;
};

export function step() {
    ea.step();

    return {
        generations: ea.num_generations(),
        evaluations: ea.num_evaluations(),
        evaluationsDelta: ea.evaluation_delta(),
        maxFitness: ea.max_fitness(),
        avgFitness: ea.avg_fitness(),
        bestPhenotype: ea.best_phenotype(),
    }
}
