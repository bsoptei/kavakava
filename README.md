# kavakava
A simple super lightweight key-value based data store


## How to compile & run
[Install cargo](https://crates.io/) if you don't have it. You can compile the project from the project folder using the command line:

```sh
cargo run --release
```

It launches a kavakava instance at 127.0.0.1:7474.

If you want to specify the address, you can do

```sh
cargo run --release [address]
```

## How to use

Simply send your message via TCP stream to kavakava. You can use any language. Here are some examples:

- [Rust](https://github.com/bsoptei/askkk/)
- [Python](https://github.com/bsoptei/askkkpy/)
- [node.js](https://github.com/bsoptei/askkkjs/)

## Tasks and syntax

### length
Gets the number of entries in the data store. Usage:

```sh
length
```

### update
Adds entries to the store, or updates existing ones. Usage:

```sh
update key1;value1;key2;value2;...keyn;valuen;
```

### delete
Removes entries the keys of which are among the arguments. Usage:

```sh
delete key1;key2;...keyn;
```

### bykeys
Gets entries based on key arguments. Usage:

```sh
bykeys key1;key2;...keyn;
```

### byvals
Gets entries based on value arguments. Usage:

```sh
byvals value1;value2;...valuen;
```

### import
Loads a file from a given path and updates the store with the contents. The file needs to have two columns in each row, separated by a chosen delimiter. No header. Usage:

```sh
import path;delimiter;
```

### export
Saves the store to a csv (if you choose a comma delimiter) or a similar format. Usage:

```sh
export path;delimiter;
```
