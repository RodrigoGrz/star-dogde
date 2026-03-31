use bevy::ecs::message::Message;

#[derive(Message)]
pub struct GameOver {
    pub score: u32,
}