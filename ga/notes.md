## GA
Current fitnesses for selection don't change probabilities that much for actual cost

multi-objective optimization - currently just does weird fitness combinations

all_cfg doesnt' work well for target_string - need to investigate.

## To implement:
1. Replace criterion.rs with custom GA hyper-param searcher / analyser

## Selection strategies
1. SUS based on fitness
2. Look for maximally different parent

## Survival strategies
1. Top proportion
2. Top proportion from each species (not implemented)

## Crossover strategies
1. k-point crossover
2. Uniform crossover
3. Partially mapped crossover (not implemented)
4. Edge crossover (not implemented)
5. Order crossover (not implemented)
6. Cycle crossover (not implemented)

## Mutation strategies
1. Single replacement - randomly replace a single gene
 - Uniform mutation, non-uniform mutation
2. Random resetting (not implemented) - randomly reset a state
3. Swap mutation (not implemented) - randomly swap two genes
4. Scramble mutation (not implemented) - scramble a substring
5. Inversion mutation (not implemented) - invert a substring
6. Creep mutation (not implemented) - add a value to gene; small creep, large creep
7. Self-adaptive mutation (not implemented)
 - Learn a mutation rate (or strategy?) in parallel

## Diversity
Adaptive mutation rate based on diversity => this is thought to be bad

## Niching
1. No niching
2. Shared fitness with species target
3. Crowding (not implemented - shared fitness generally better)

## Measures of performance
1. Best fitness of last generation
2. Mean fitness of last generation
3. Number of duplicate states in last generation
4. Mean distance between states
5. Number of species
6. Number of runs to a solution (not implemented)

## Hyper-parameter tuning
1. Meta-GA (not implemented)
 - See SPO, F-race, REVAC, meta-GA
2. Use tuning search method to analyse robustness of GA
3. Tuning numeric params (e.g. mutation rate) vs symbolic (e.g. selection method)
 - Robust set of symbolic params => works well for a large set of numeric params.

## Example problems
1. Target string evolution
2. Knapsack
3. Shortest path (not implemented)
4. Travelling salesperson (not implemented)

## Findings
Stochastic Universal Selection vs Roulette Wheel Selection:
On 'hello world':
RWS:
 - worse #runs avg and distribution to convergence
 - worse mean fitness
 - fewer duplicates
