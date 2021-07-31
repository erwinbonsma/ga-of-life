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
        max_fitness: ea.max_fitness()
    }
}
