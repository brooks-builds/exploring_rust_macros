use bbecs::get_resource;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};

// macro_rules! hello_world {

//     () => {
//         println!("hello unknown");
//     };

//     ($($name:expr), *) => {
//         $(
//             println!("Hello {}", $name);
//         )*
//     };
// }

// #[allow(unused_macros)]
// macro_rules! get_resource {
//     ($resource:ident, $world:expr, $name:expr) => {
//         let wrapper = $world.get_resource($name).unwrap().borrow();
//         $resource = wrapper.cast().unwrap();
//     };
// }

// macro_rules! get_mut_resource {
//     ($name:expr, $world:expr, $type:ty, $wrapper:ident) => {{
//         $wrapper = $world.get_resource($name).unwrap().borrow_mut();
//         let resource: &mut $type = $wrapper.cast_mut().unwrap();
//         resource
//     }};
// }

macro_rules! get_mut_resource {
    ($resource:ident, $world:expr, $name:expr) => {
        let mut wrapper = $world.get_resource($name).unwrap().borrow_mut();
        $resource = wrapper.cast_mut().unwrap();
    };
}

#[allow(clippy::float_cmp)]
fn main() {
    let mut world = bbecs::world::World::new();
    world.add_resource("size".to_string(), 15.0_f32);

    // let wrapped_resource = world.get_resource("size").unwrap().borrow();
    // let resource: &f32 = wrapped_resource.cast().unwrap();
    // let resource = get_resource!("size", world, f32);
    // println!("The size is {}", resource);

    // let mut wrapped_resource;
    // let resource = get_mut_resource!("size", world, f32, wrapped_resource);

    // *resource += 1.0;
    // let size = *resource;
    // drop(wrapped_resource);

    // let resource_again = get_resource!("size", world, f32);
    // assert!(size == resource_again);
    update_size(&mut world);
    let size: &f32;
    get_resource!(size, world, "size");
    dbg!(size);
}

fn update_size(world: &mut World) {
    let mut_size: &mut f32;
    get_mut_resource!(mut_size, world, "size");

    dbg!(&mut_size);

    *mut_size += 1.0;
}
