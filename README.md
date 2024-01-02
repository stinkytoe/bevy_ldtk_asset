
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/stinkytoe/bevy_ldtk_asset/tree/main#license)

## What is the Bevy LDtk Loader?

I am developing the Bevy LDtk Loader as a tool to load LDtk Levels into the Bevy game engine!

There are many projects out there which aim to do this as well. 

## Philosophy

I aim to create a library for users to:
- Load levels and/or entity instances from an LDtk project file, as Bevy assets
- Create ECS entities representing these levels
- Have these display to the screen in a simple and expected way
- Provide hooks for the user to work with these elements

What this library is NOT intended to do:
- Define how things like the following should work:
-- Collision Detection
-- Sprite animations
-- Parallax backgrounds
-- Any other logic which a game may need

## Levels

Levels are loaded as ECS entities with their translation set to their location in the project world space. They will
be spawned with the following children ECS entities, in ascending Z order:
- A mesh spanning the level coordinates set to the LDtk background color 
- If one is defined, a mesh containing the background image. This will be cropped and scaled as defined in the LDtk level definition
- Each layer which is defined for this level instance

Level assets contain a value field, which is the rust representation of the JSON data which was read from the LDtk project. 

## Layers

Layers are loaded and added to the world as ECS entities, with their respective visual representation attached (per layer type). 

### Tile layer

A tile layer will contain a `Handle<Image>` referencing its visual contents. 

### Entity Layer

An entity layer will include, as children ECS entities, all the entity instances defined in that layer. These
ECS entities will include an `LdtkEntityComponent` which the user can query and react against. 

If an editor visual is defined, we will attempt to render by spawning either a Sprite or a MaterialMesh2d as a child
of the main ECS entity. We don't render shapes, or alt UI icon visuals, but the info is provided in the value field 
if you want to render them yourselves. 

## What's working

Currently a user can load a level as a labeled asset of an LDtk project file. As an example,
adding the following to a typical startup system in a Bevy project will add the defined layers and entity instances
to the world:

```rust
    commands.spawn(LdtkLevelBundle {
        level: asset_server.load("ldtk/example.ldtk#Level_0"),
        ..default()
    });
```

### LDtk file requirements

Over time I will do my best to include as many optional features of LDtk as are appropriate. Right now, these 
requirements of the LDtk project are in place:
- Multi-world is not yet supported 
- External level files is not yet completed
- It is required to select the "One PNG per layer" or "One PNG per layer and one per level" option in the Project settings
- Only "Entities" and "Tiles" layers are loaded, the others are silently ignored
- Only the following entity instance editor visual types are supported:
-- "Dirty stretch to bounds"
-- "Fit inside bounds"

## License 

Released under the dual MIT and Apache licenses, as inspired by the Bevy team. 

## Getting Started

Add bevy_ldtk_asset to your Config.toml file, along with Bevy. We currently target Bevy version 0.12. 

## Included Assets

[Fantasy Battle Pack](https://mattwalkden.itch.io/fantasy-battle-pack)
[Treasure Hunters](https://pixelfrog-assets.itch.io/treasure-hunters)
