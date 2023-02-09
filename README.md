# FYSX
Small physics particle simulation, based on verlet

# Things to adjust
(find with Ctrl+F)
[simulation.rs](src/simulation.rs)
- `gx`/`gx` - gravity
- `sub_steps` - stability/performance
- `response_coef` - stability/springiness
- `min_dist_sq` - stability/performance