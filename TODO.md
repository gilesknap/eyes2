TODO
====

Stage one
---------

to create the framework for evolving creatures:

- DONE create a tui based representation of the world with GUI for controlling settings/tweaks
- provide a creature implementation plugin architecture so that the decisions creatures
  make can be 'intelligent' and inheritable
- DONE provide a 'dumb' creature plugin with deterministic behaviour for perf testing and tui
  testing
- provide means to implement carnivores and herbivores
- DONE provide grass that grows with variable rate to supply stress to the ecosystem
- implement some barriers
- implement editors to set up barriers and maybe initial state of creatures / grass
- DONE provide a mechanism for slowing down or stopping activity (for visualization)
- DONE provide a standard set of entity interactions with the world to
  - move
  - create new entity
  - eat another creature / grass
  - remove self
  - NOT DONE look at nearby cells
- STRETCH: implement multi-threaded architecture with current entities distributed in threads
- NIRVANA: implement multi host architecture and deploy with kubernetes
    - I think I won't do this. Distributing across processes for the trivial work a creature does
      will not scale
- STRETCH provide a debug architecture
  - needs to implement a GUI for representing the state of the creature
  - e.g. assembler / disassembler and debugger

Stage 2
-------

write some creature plugins

- invite collaborators to compete for best evolving algorithms