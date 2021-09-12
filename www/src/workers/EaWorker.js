let wasm;
let wasm_bg;

let ea;

// A bit of a pain to explictly initialize WASM object this way, but there does not seem a much
// nicer solution, given that the redux state should be immutable.
function settings_to_wasm(settings) {
    const ea_settings = new wasm.MyEaSettings()
        .set_border(settings.borderWraps)
        .set_mutation_rate(settings.mutationRate)
        .set_recombination_rate(settings.recombinationRate)
        .set_population_size(settings.populationSize)
        .set_tournament_size(settings.tournamentSize)
        .set_elitism(settings.elitism)

    console.log({ settings });

    return ea_settings;
}

export async function init(settings) {
    if (!wasm) {
        wasm = await import('ga-of-life');
    }
    if (!wasm_bg) {
        wasm_bg = await import('ga-of-life/ga_of_life_bg.wasm');
    }

    ea = new wasm.MyEvolutionaryAlgorithm(settings_to_wasm(settings));

    return ea;
};

export function reset(settings) {
    ea.reset(settings_to_wasm(settings));
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
        geneDistribution: new Float32Array(wasm_bg.memory.buffer, ea.gene_distribution(), ea.genotype_len()),
        cellDistribution: new Float32Array(wasm_bg.memory.buffer, ea.cell_distribution(), ea.phenotype_len()),
    }
}
