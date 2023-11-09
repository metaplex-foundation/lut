## LUT - A simple CLI for creating and managing Solana Lookup Tables

## Commands

### Create

Creates a new LUT using the default keypair in the Solana config file.

```bash
lut create 
```

---

### Extend

Appends new addresses to the LUT.

From space-separated addresses on the command line:

```bash
lut extend <lut_address> -a <address1> -a <address2> -a <address3>
```
Or from a JSON file with a list of addresses:

```bash
lut extend <lut_address> -f <path_to_json_file>
```

---

### Decode
Decode all current entries in the LUT and print to the command line.
    
    ```bash
    lut decode <lut_address>
    ```

---

### Deactivate
Deactivates the LUT, starting the cooldown period.
```bash
lut deactivate <lut_address>
```

---

### Close
Closes the LUT and returns the rent funds to the owner keypair.


```bash
lut close <lut_address>
```
