
let ea;
let memory;
export async function init() {
    if (!ea) {
        const wasm = await import('ga-of-life');
        ea = new wasm.MyEvolutionaryAlgorithm();

        const wasm_bg = await import('ga-of-life/ga_of_life_bg.wasm');
        memory = wasm_bg.memory;
    }

    return ea;
};

export function reset() {
    ea.reset();
}

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
        geneDistribution: new Float32Array(memory.buffer, ea.gene_distribution(), ea.genotype_len()),
        cellDistribution: new Float32Array(memory.buffer, ea.cell_distribution(), ea.phenotype_len()),
    }
}
