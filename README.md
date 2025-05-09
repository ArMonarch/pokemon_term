# Pokemon Term (Poke)
A program to print out images of pokemon to the terminal. Rewrite of [Phoney badger pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts) in Rust.

## Features

  - Every generation Pokemon, including shinies, megas, gigantamax and regional varient.
  - List all the Pokemons. Along with their forms.
  - Print Pokemon by name.
  - Print the shiny version of the given Pokemon.
  - Print random Pokemon (Shiny and Different Forms).

## Usage
```
Usage:
  poke -l | --list
  poke -n | --name (pokemon_name)
  poke -h | --help
  poke -v | --version

Arguments:
  -n, --name=NAME                   Print the Pokemon by its Name. Generally spelled like in the game.
  -l, --list                        Print a list of all pokemons
  --show-forms                      Show List of Pokemons with their respective forms.
  -s, --shiny                       Print the shiny version of the pokemon.
  -f, --form=FORM                   Print the given form version of the pokemon.
  -r, --random                      Print a Random Pokemon in the terminal. Includes shiny version and their forms.
  --random-by-name=[Pokemon Names]  Print Random Pokemon from given Pokemon names. Pokemon names must be seperated by comma(',').
  --random-by-gen=Generation        Print Random Pokemon from given Generations. Generation value , 1-3(continious) 1,3,5(specific).
```

## Installation

------------------------------------------------------

## Similar projects
- [Krabby]()
- [pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts)
- [pokeget](https://github.com/talwat/pokeget)
- [pokeshell](https://github.com/acxz/pokeshell)
