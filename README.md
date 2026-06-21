# eyes2 - An evolution simulator

## Intro
This is my learn rust project and is a re-imagining of something I made over
20 years ago.

The idea is to provide a 'world' which is a 2d grid of cells each of which
can contain empty space, a creature or some grass.


https://user-images.githubusercontent.com/964827/218594561-a8dfc5e1-45ce-4b0d-98cc-6b754de4c00e.mp4



## World Rules

The creatures have a certain amount of energy and can request interactions
with the world as follows:

- Look: request the values of adjacent cells in the world
- Move: move one step in any direction.
- Reproduce: split your energy and make a copy of yourself

The world enforces some rules on the creatures:

- When a herbivore lands on grass it gains energy and the grass is destroyed.
- When a carnivore lands on a herbivore it takes all its energy
- When energy is 0 the creature dies
- Grass has as certain growth rate, but only grows from existing grass cells

## The Genetic Code

Each creature will have a genome of some kind and this will be used to determine
its behaviour. On reproduction the genome is copied to the child with a
mutation rate.

The original version had a single implementation of the genotype and it looked
a lot like a randomly initialized block of RISC instructions with a few registers
some of which where inputs and outputs to the world.

This version did successfully evolve useful survival strategies in which I
was able to turn the grass growth rate down much lower than normal values.
(at high enough grass growth, random behaviour is sufficient for survival)

I have now reproduced that same 'genetic code' as the `giles` genotype. Each
creature carries a 1000 byte genome that is interpreted as byte-code for a tiny
virtual machine, running one instruction per world tick. The VM has 12
instructions (LOADC, LOADV, ANDV, ORV, JZ, JNZ, MOVV, MOVC, NOP, SAVEV, ADDV,
SUBV), a single accumulator register `r`, five I/O registers (I1..I5) and an
instruction pointer. Readable variables include the 8 directional vision inputs
(V1..V8), energy (E), dead-reckoning position (X, Y), the breed threshold (B),
the mutation rate (M) and the five I/O registers. On reproduction the genome is
copied to the child and, with probability equal to the creature's own (and
therefore evolvable) mutation rate, the copy is mutated - per-byte flips plus a
roving copy point that gives gene duplication and rearrangement. The breed
threshold and mutation rate are themselves part of the genome and so evolve too.

One difference from the original: `giles` uses eyes2's existing single
adjacent-cell Look vision (cached and refreshed after each move) rather than the
original's 4-cell-deep ranged vision.

This version is also extensible, so multiple types of creature with different
genetic codes may co-exist.

## Goals

- try to use as many nice features of rust as possible
- make this very high performance - the more iterations the more chance of
  interesting evolution happening

## How to add a new Genotype

This currently requires code changes but maybe we could do dynamic crate
loading in future.

- in src/entity/genotype/genotypes take a copy of random.rs, rename it
  and replace RandomGenotype to YourNewGenotype or similar
- add pub mod your_new_module to src/entity/genotype/genotypes/mod.rs
- add an extra arm to the match in src/entity/genotype/genotype::new_genotype
- maybe add some of your new type into the default settings in
  src/settings.rs

That's it. Now you can start to make your own custom genetic code.
The `giles` genotype (in genotypes/giles.rs) is a fully worked example of this
that you can look at.

# Still to do

- DONE Save and Restore of worlds and individual creatures
- Creature Vision
- Barriers (thanks Michael Abbott) - add some barriers that stop creature
  movement - introducing extra environmental challenges (or advantages perhaps).
  Provide the means to edit the location of barriers in the world.
- DONE Implementation of the original RISC Genotype (now the `giles` genotype)
- Get some competing Genotype contributions and have some creature wars
- Carnivores
- Multi Threaded processing for the creatures for even more performance
  (may be hard as we currently loop over all and call one tick - this
  model would need to change)
- Multi processing and a helm chart to deploy into K8S - major remodel would
  be required
