use bevy::prelude::*;

const CHUNK_SIZE: usize = 32;

/* Implementing blocks
 *
 * Blocks need: A cube (this stores absolute position in the world and size), one to six textures,
 * knowledge of their adjacent blocks. The GUID of each block is handled by their `Entity`
 * instance.
 *
 * Okay, here's where I get stuck: I have no idea how to build a continuous mesh that you can
 * change at runtime.
 *
 * The internet seems to have this answer: have the cubes track what is adjacent to them. If it's
 * an air block, a fluid block, or a smaller block (like a fence, a ladder, a sign...) then render.
 * Otherwise, don't render that face. Simple pattern matching can achieve this very nicely :)
 *
 * Of course we group voxels into chunks, since constant memory or disk accesses will certainly
 * slow things down. 32x32x32 seems to be a common size, so for simplicity's sake we'll go with
 * that.
 *
 * But this introduces a new problem: what happens when the player is on a chunk boundary?
 * Does each chunk, therefore, also need to be aware of its adjacents?
 *
 * For now, they will.
 *
 * When implementing placement and destruction of blocks, we'll need to do a cheeky raycast from
 * the center of the screen to le block.
 *
 * Repr in ECS:
 * VoxelWorld: ResMut -- Stores HashMap<IVec3, VoxelChunk>
 *
 * */

// This might introduce a massive problem of shared mutable references...
// Or not. Blocks will always be destroyed/placed sequentially, even if it's in less than a frame,
// perhaps we get_mut(block_id) then get that reference outta scope?

// AAAAGH FORGET ALL THIS! Let's just try getting a chunk in our world first.

// Component? Bundle?


// This compiles just fine. As we can clearly see by this comment getting highlighted as a syntax
// problem, rust analyzer just can't understand this macro.
#[derive(Bundle)]
pub struct VoxelBundle {
    #[bundle]
    pub cube_mesh: PbrBundle,
    pub state_of_matter: StateOfMatter,
    // pub adjacents: [Option<u64>; 6], // Store the entity UUID for the sake of simplicity and my sanity (borrow checker)
}

struct Voxel {
    state_of_matter: StateOfMatter,
    adjacents: [Option<u64>; 6],
}

/*
struct VoxelChunk {
    voxels: [[[Voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    // adjacents: [Option<u64>; 6],
}
*/
/*
impl Default for VoxelChunk {
    fn default() -> Self {
        Self {

        }
    }
}
*/ 

// Implment this after you get chunks rendering.
enum _VoxelTexture<'handle> {
    None,
    Uniform(usize), // Index of texture in atlas
    // Minecraft blocks can have a varying number of different textures depending on the block.
    MultiFace(&'handle [usize]),
}

// Use as C-like enum.
enum _CubeFace {
    Up,
    Front,
    Right,
    Back,
    Left,
    Bottom,
}



// TODO: Add liquids
pub enum StateOfMatter {
    Solid,
    Gas,
}

// Region: Resources

struct _Materials {
    block_atlas: Handle<TextureAtlas>,
}

// This will become necessary once we start building more world past startup
// struct _VoxelWorld(std::collections::HashMap<IVec3, VoxelChunk>);

