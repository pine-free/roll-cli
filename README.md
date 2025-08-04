# Roll-cli
## A dice-roller for all my nerdy needs

### Usage
You can roll any dice you want in any numbers (within the unsigned integer limit,
but if you need that many dice I think you've got bigger problems)
```console
$ roll-cli "1d12"  # Regular die
$ roll-cli "1d7"  # Not-so regular die
$ roll-cli "3d8"  # Multiple dice
$ roll-cli "4d6 + d4"  # Multiple dice of different types
$ roll-cli "4d6 + d4 + 3" -r  # Counting the result ft. numbers 
```

### Installation

#### Nix flakes

Add this into your system configuration or modify it accordingly

```nix
# flake.nix

{...}: {
  inputs = {
    roll-cli.url = "github:pine-free/roll-cli";
  };
  outputs = { ... }@inputs: {
     # <snip>
  }
```

```nix
# configuration.nix
{...}: {
  environment.systemPackages = [
    inputs.roll-cli.defaultPackage.<your-architecture>
  ];
}
```
