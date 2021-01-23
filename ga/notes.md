## GA
Current fitnesses for selection don't change probabilities that much for actual cost

multi-objective optimization - currently just does weird fitness combinations

## Selection strategies
1. SUS based on fitness
2. Look for maximally different parent

## Crossover strategies
1. k-point crossover
2. Uniform crossover

## Mutation strategies
1. Single replacement - randomly replace a single bit
2. Random resetting (not implemented) - randomly reset a state
3. Swap mutation (not implemented) - randomly swap two bits
4. Scramble mutation (not implemented) - scramble a substring
5. Inversion mutation (not implemented) - invert a substring

## Diversity
Adaptive mutation rate based on diversity => this is thought to be bad


## Niching
See https://arxiv.org/pdf/1508.05342.pdf

Island model GAs - migration between

Fitness sharing
fitness points shared with neighbours within a certain radius -> can create species and dynamically
adjust distance

sequential fitness sharing - see http://www.cse.cuhk.edu.hk/~ksleung/download_papers/Adaptive_population_%20App_Soft_Comp_2011.pdf


crowding

## Measures
Problem: Evolving string to hello world - seems bad, improve

1. Best fitness of last generation
2. Mean fitness of last generation
3. Number of duplicate states in last generation
4. Mean distance between states

## Findings
Stochastic Universal Selection vs Roulette Wheel Selection:
On 'hello world':
RWS:
 - worse #runs avg and distribution to convergence
 - worse mean fitness
 - fewer duplicates
