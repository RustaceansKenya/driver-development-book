# Core::mem

Just go read the docs to get the full picture.  

Two ways to use this crate.  
1. To perform little tricks that can be done even without this crate. (discussed below under "Tricks").
2. To perform analysis and manipulation of how data-types are laid out in memory. (discussed below under "memory_view")


### Tricks
You can :  
- `replace` a value : Moves src into the referenced dest, returning the previous dest value.  
- `swap` two values in memory - swap two values in memory without without deinitializing either one.
- replace a value with a `default` value and `take` the old value