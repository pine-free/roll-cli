# Roll-cli
## A dice-roller for all my nerdy needs

### Usage
You can roll any dice you want in any numbers (within the unsigned integer limit,
but if you need that many dice I think you've got bigger problems)
```console
$ roll-cli "1d12"  # Regular dice roll
$ roll-cli "4d6 + 1d4 + 3 - 1d8"  # Basic calculations 
$ roll-cli "hp: 3d6; arrows in pouch: 4d4 + 6"  # Custom labels, several expressions in one
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
