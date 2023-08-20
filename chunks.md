## Chunk geometry
Even tho we want to extend the world infinitely in every directions, it makes more sense to have non-cubic chunk shapes. The one that makes the most sense to me is a column shaped block for the following reasons:
- It makes more sense for terrain generation
- It's useful to extend the chunk size on one axis but it makes no sense to do it on the x or z axis
- The player will less often travel vertically so it makes sense for this axis to be the most expensive one
- Works like other famous voxel engines (and Minecraft)


