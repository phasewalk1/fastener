# Usage

__Install__
```fish
cargo install --path .
```

### The Git Clone Wrapper
The below is an example of using the `git clone` wrapper for more concise clone comands.
```fish
fastener phasewalk1/cayde
```

### Using the [Yay](https://github.com/Jguer/yay) Wrapper
`fastener` will automatically run in `yay` mode if the pattern of the package matches what `yay` expects, e.g., no slash is contained.
```fish
fastener dwm
```

### Future Wrappers
`fastener` will, in the future, contain additional concise wrappers for routine dev commands such as but not limited to:
```fish
git remote add origin <_>
```
```fish
make clean install
```