# Setting up a devenv

1. Install [nix](https://github.com/DeterminateSystems/nix-installer) (or install the required dependencies yourself, but for e.g. tests, there are a lot)
2. Install [direnv](https://direnv.net/) (optional, recommended for ease of use)
3. Run `direnv allow` / `nix develop --impure`

### Tests
Note: I did not manage to set up all dependencies yet - e.g. java tests are failing
```bash
# enter shell:
nix develop --impure .#test
# run directly:
nix develop --impure .#test -c cargo test
```