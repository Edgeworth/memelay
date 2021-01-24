## GA
Current fitnesses for selection don't change probabilities that much for actual cost

multi-objective optimization - currently just does weird fitness combinations

## To implement:
1. shared fitness niching
2. self-adaptive mutation rate

## Selection strategies
1. SUS based on fitness
2. Look for maximally different parent

## Crossover strategies
1. k-point crossover
2. Uniform crossover

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

See https://arxiv.org/pdf/1508.05342.pdf

Island model GAs - migration between

Fitness sharing
fitness points shared with neighbours within a certain radius -> can create species and dynamically
adjust distance

sequential fitness sharing - see http://www.cse.cuhk.edu.hk/~ksleung/download_papers/Adaptive_population_%20App_Soft_Comp_2011.pdf


crowding

## Measures
1. Best fitness of last generation
2. Mean fitness of last generation
3. Number of duplicate states in last generation
4. Mean distance between states

## Problems
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
