# Block Registry Dumper 🦀

This program uses `tree-sitter` to parse Minecraft's `Blocks.java` script and output block behaviour info that isn't available via the game's JSON files, VERY fast.

<details>
<summary>Input Example</summary>

*The file has been truncated for simplicity.*

```java
package net.minecraft.world.level.block;

import java.util.function.Function;
import java.util.function.ToIntFunction;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.Registry;
import net.minecraft.core.cauldron.CauldronInteraction;
import net.minecraft.core.particles.ColorParticleOption;
import net.minecraft.core.particles.ParticleTypes;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.data.worldgen.features.CaveFeatures;
import net.minecraft.data.worldgen.features.TreeFeatures;
import net.minecraft.data.worldgen.features.VegetationFeatures;
import net.minecraft.references.Items;
import net.minecraft.resources.Identifier;
import net.minecraft.resources.ResourceKey;
import net.minecraft.sounds.SoundEvents;
import net.minecraft.util.ColorRGBA;
import net.minecraft.util.valueproviders.ConstantInt;
import net.minecraft.util.valueproviders.UniformInt;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.item.DyeColor;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.biome.Biome;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraft.world.level.block.entity.BlockEntityType;
import net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity;
import net.minecraft.world.level.block.entity.trialspawner.TrialSpawnerState;
import net.minecraft.world.level.block.entity.vault.VaultState;
import net.minecraft.world.level.block.grower.TreeGrower;
import net.minecraft.world.level.block.piston.MovingPistonBlock;
import net.minecraft.world.level.block.piston.PistonBaseBlock;
import net.minecraft.world.level.block.piston.PistonHeadBlock;
import net.minecraft.world.level.block.state.BlockBehaviour;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.BedPart;
import net.minecraft.world.level.block.state.properties.BlockSetType;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;
import net.minecraft.world.level.block.state.properties.NoteBlockInstrument;
import net.minecraft.world.level.block.state.properties.SculkSensorPhase;
import net.minecraft.world.level.block.state.properties.WoodType;
import net.minecraft.world.level.material.Fluids;
import net.minecraft.world.level.material.MapColor;
import net.minecraft.world.level.material.PushReaction;

public class Blocks {
    public static final Block ACACIA_PLANKS = register(
        "acacia_planks",
        BlockBehaviour.Properties.of()
            .mapColor(MapColor.COLOR_ORANGE)
            .instrument(NoteBlockInstrument.BASS)
            .strength(2.0F, 3.0F)
            .sound(SoundType.WOOD)
            .ignitedByLava()
    );
   public static final Block CHERRY_PLANKS = register(
      "cherry_planks",
      BlockBehaviour.Properties.of()
         .mapColor(MapColor.TERRACOTTA_WHITE)
         .instrument(NoteBlockInstrument.BASS)
         .strength(2.0F, 3.0F)
         .sound(SoundType.CHERRY_WOOD)
         .ignitedByLava()
   );
   public static final Block DARK_OAK_PLANKS = register(
      "dark_oak_planks",
      BlockBehaviour.Properties.of()
         .mapColor(MapColor.COLOR_BROWN)
         .instrument(NoteBlockInstrument.BASS)
         .strength(2.0F, 3.0F)
         .sound(SoundType.WOOD)
         .ignitedByLava()
   );
   public static final Block PALE_OAK_WOOD = register(
      "pale_oak_wood",
      RotatedPillarBlock::new,
      BlockBehaviour.Properties.of().mapColor(MapColor.STONE).instrument(NoteBlockInstrument.BASS).strength(2.0F).sound(SoundType.WOOD).ignitedByLava()
   );
   public static final Block PALE_OAK_PLANKS = register(
      "pale_oak_planks",
      BlockBehaviour.Properties.of().mapColor(MapColor.QUARTZ).instrument(NoteBlockInstrument.BASS).strength(2.0F, 3.0F).sound(SoundType.WOOD).ignitedByLava()
   );
   public static final Block MANGROVE_PLANKS = register(
      "mangrove_planks",
      BlockBehaviour.Properties.of()
         .mapColor(MapColor.COLOR_RED)
         .instrument(NoteBlockInstrument.BASS)
         .strength(2.0F, 3.0F)
         .sound(SoundType.WOOD)
         .ignitedByLava()
   );
   public static final Block BAMBOO_PLANKS = register(
      "bamboo_planks",
      BlockBehaviour.Properties.of()
         .mapColor(MapColor.COLOR_YELLOW)
         .instrument(NoteBlockInstrument.BASS)
         .strength(2.0F, 3.0F)
         .sound(SoundType.BAMBOO_WOOD)
         .ignitedByLava()
   );
   public static final Block BAMBOO_MOSAIC = register(
      "bamboo_mosaic",
      BlockBehaviour.Properties.of()
         .mapColor(MapColor.COLOR_YELLOW)
         .instrument(NoteBlockInstrument.BASS)
         .strength(2.0F, 3.0F)
         .sound(SoundType.BAMBOO_WOOD)
         .ignitedByLava()
   );
   public static final Block OAK_SAPLING = register(
      "oak_sapling",
      $$0x -> new SaplingBlock(TreeGrower.OAK, $$0x),
      BlockBehaviour.Properties.of()
         .mapColor(MapColor.PLANT)
         .noCollision()
         .randomTicks()
         .instabreak()
         .sound(SoundType.GRASS)
         .pushReaction(PushReaction.DESTROY)
   );
   public static final Block SPRUCE_SAPLING = register(
      "spruce_sapling",
      $$0x -> new SaplingBlock(TreeGrower.SPRUCE, $$0x),
      BlockBehaviour.Properties.of()
         .mapColor(MapColor.PLANT)
         .noCollision()
         .randomTicks()
         .instabreak()
         .sound(SoundType.GRASS)
         .pushReaction(PushReaction.DESTROY)
   );
}
```
</details>

<details>
<summary>Output Example</summary>

*The file has been truncated for simplicity.*

```json
[
    "acacia_planks": {
        "soundType": "WOOD"
    },
    "cherry_planks": {
        "soundType": "CHERRY_WOOD"
    },
    "dark_oak_planks": {
        "soundType": "WOOD"
    },
    "pale_oak_wood": {
        "soundType": "WOOD"
    },
    "pale_oak_planks": {
        "soundType": "WOOD"
    },
    "mangrove_planks": {
        "soundType": "WOOD"
    },
    "bamboo_planks": {
        "soundType": "BAMBOO_WOOD"
    },
    "bamboo_mosaic": {
        "soundType": "BAMBOO_WOOD"
    },
    "oak_sapling": {
        "soundType": "GRASS",
        "instabreak": true,
        "noCollision": true
    },
    "spruce_sapling": {
        "soundType": "GRASS",
        "instabreak": true,
        "noCollision": true
    }
]
```
</details>

## Usage

1. Use a tool like [Minecraft Decompiler](https://github.com/MaxPixelStudios/MinecraftDecompiler) to decompile Minecraft.
2. Find the Blocks script - its path should be `<VERSION>-decompiled\net\minecraft\world\level\block\Blocks.java`.
3. Drag it into the current working directory - making sure to keep its name.
4. Run the tool! It should create a `block_registry_out.json` file.

## Similar Tooling

- [Burger](https://github.com/TkTech/Burger)
- [minecraft-data](https://github.com/PrismarineJS/minecraft-data) (no block sound types, but provides lots of useful data)](https://cataas.com/cat)
