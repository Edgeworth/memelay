## GA
Current fitnesses for selection don't change probabilities that much for actual cost

multi-objective optimization - currently just does weird fitness combinations

## Selection strategies
SUS based on fitness
look for maximally different parent

## Diversity
Adaptive mutation rate based on diversity => this is thought to be bad

some sort of entropy based diversity measure?

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

1. Number of runs to convergence
2. Mean fitness of last generation
3. Number of duplicate states in last generation

## Findings
Stochastic Universal Selection vs Roulette Wheel Selection:
On 'hello world':
RWS:
 - worse #runs avg and distribution to convergence
 - worse mean fitness
 - fewer duplicates
