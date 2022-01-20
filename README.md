# Bevy Bistro Playground

Bevy playground with the Bistro scene from https://developer.nvidia.com/orca/amazon-lumberyard-bistro

![](bevy_bistro_playground.png)

## How to use

Download the scenes, and reexport `BistroExterior.fbx` and `BistroInterior_Wine.fbx` as GLTF files (in `.glb` format). Move the glb files to the assets folder.

## What it does

Both scenes will be loaded, with a few modifications:
* Interior:
  * Front door is removed as it is not perfectly aligned with front door of exterior scene
  * Point lights are spawned on the ceiling lights
* Exterior:
  * Front door glass is made transparent
  * Streetlight glass is made transparent
  * Point lights are spawned on the street lights
  * Point lights are spawned on the lanterns

With all lights added, there are 21 point lights.

In a real game, those changes should be done on the scenes themselves before being loaded in Bevy. This is done in Bevy here to work with the original scenes without modifications on them.

A directional light is added that will change direction based on the time, and the ambient light vary with the angle of the directional light. This simulates a day/night cycle.

## Known issues

The normals are wrong, as they are not in the expected format by Bevy. This is easily fixed in the exported scene.
