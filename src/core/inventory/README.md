# core/inventory
The core TS implementation of BOTW inventory

There are 2 classes here: `Slots` and `SlotsCore`

`SlotsCore` is a wrapper around a `ItemStack[]` that implements basic methods

`Slots` contains more complex functions such as add, remove, equip, etc that uses one or more core method

Ideally, methods in `Slots` should all have unit tests. Make sure unit tests are added or changed accordingly when changing `Slots`

## Note
All branches in public interface should have comment with `[confirmed]` or `[need confirm]` indicating whether something is confirmed to be the same in game
- cases tagged with `[confirmed]` must also have unit tests covering them
- make sure to add unit tests when changing from `[needs confirm]` to `[confirmed]`
