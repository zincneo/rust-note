/*!
# Entity-Component-System

ECS is a software pattern that involves breaking your program up into Entites, Components, and Systems. Entities are unique "things" that are assigned groups of Components, which are then processed using Systems.

For exmaple, one entity might have a `Position` and `Velocity` component, whereas another entity might have `Position` and `UI` component. You might have a movement system that runs on all entites with a Position and Velocity component.

The ECS pattern encourages clean, decoupled designs by forcing you to break up your app data and logic into its core components. It also helps make your code faster by optimizing **memory access** patterns and making **parallelism** easier.

*/
pub mod bevy_case;
