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

## Niching
1. No niching
2. Shared fitness with species target
3. Crowding (not implemented - shared fitness generally better)

## Fitness evaluation
1. Stepwise adaption of weights (not implemented)
 - As time goes on, add increasing penalties to particular constraints

## Multiobjective optimisation (MOEA)
Approaches:
1. Assign fitness based on # members dominated + fitness sharing
2. Repeatedly take the pareto front and assign fitness based on iteration found
3. MOEA-D

## Constraint handling
1. Stepwise Adaption of Weights penalty
 - If best solution violates constraint i, it is a hard constraint so increase the penalty factor.

## Measures of performance
1. Best fitness of last generation
2. Mean fitness of last generation
3. Number of duplicate states in last generation
4. Mean distance between states
5. Number of species
6. Number of runs to a solution (not implemented)

## Tuning / analysis
1. Graph of GA progress + mean progress averaged over multiple runs
2. Statistical and graph comparison of two GAs
3. ANOVA test - statistical analysis of varying multiple parameters
4. Two-tailed t-test

## Hyper-parameter tuning
- Meta-GA (not implemented)
  Multi-objective
  Optimisations: Sharpening, Racing
- Adaptive mutation and crossover rates
- Multiple crossover and mutation methods with adaptively evolved rates (not implemented)

## Extra stuff
1. Local search (not implemented)?
2. Choice between minimisation and maximisation
 - 1/(1 + f(x))

## Example problems
- Target string evolution
- Knapsack
- Ackley function
- Griewank function
- Rastrigin function
- Travelling salesperson (not implemented)
- Shortest path (not implemented)

## Findings
Stochastic Universal Selection vs Roulette Wheel Selection:
On 'hello world':
RWS:
 - worse #runs avg and distribution to convergence
 - worse mean fitness
 - fewer duplicates
