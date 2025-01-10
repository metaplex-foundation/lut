## LUT - A simple CLI for creating and managing Solana Lookup Tables

## Commands

### Create

Creates a new LUT using the default keypair in the Solana config file as the authority.

```bash
lut create
```

### Extend

Appends new addresses to the LUT from the command line and/or a JSON file.

```bash
lut extend <lut_address> -a <address1> -a <address2> -a <address3> -f addresses.json
```

### Deactivate

Deactivates the LUT, starting the cooldown period.

```bash
lut deactivate <lut_address>
```

### Close

Closes the LUT and returns the rent funds to the owner keypair.

```bash
lut close <lut_address>
```
