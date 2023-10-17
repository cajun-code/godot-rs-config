use godot::prelude::*;
use godot::engine::Area2D;
use godot::engine::Area2DVirtual;

#[derive(Debug, GodotClass)]
#[class(base=Area2D)]
struct Player{

    #[base]
    body: Base<Area2D>,
}

#[godot_api]
impl Area2DVirtual for Rocket{
    fn init(base: Base<Area2D>)->Self{
        Self { body: base }
    }
    
}