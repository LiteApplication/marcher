# marcher
A raymarching program that uses signed distance fields to visualize objects

## Building and running
This is a rust codebase, so it's pretty easy to build and run the program :
```sh
git clone https://github.com/LiteApplication/marcher
cd marcher
cargo run --release # or without the --release to build and run the debug version
```

## Pipeline

1. Object list 
  Translate the scene description into a list of objects, each one having a material
2. Primitive expansion
  Split the object into its primitive shapes (forming a tree)
3. Materials list building
  Pack all the materials used in the scene into a C-style array to put directly in the GLSL code 
4. Object function creation
  Transform the tree of each object into a GLSL Signed Distance Field function
5. Scene function creation
  Put all the object funcitons into a scene function to be able to both raymarch and identify individual hits on an object
6. Template filling
  Use the results from above to combine everything into a compilable shader that is then sent to the GPU

For now, the program only uses a fragment shader for the rendering. the whole scene is rendered onto a triangle that fills the whole screen
