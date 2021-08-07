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
        caSteps: ea.num_ca_steps(),
        caStepsDelta: ea.ca_steps_delta(),
        maxFitness: ea.max_fitness(),
        avgFitness: ea.avg_fitness(),
        bestGenotype: ea.best_genotype(),
        bestPhenotype: ea.best_phenotype(),
        //geneDistribution: ea.gene_distribution(),
    }
}
