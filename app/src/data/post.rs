use bevy_ecs::component::Component;
// use bevy_ecs::bundle::Bundle;

#[derive(Component, Clone)]
pub struct PostTitle(pub String);

#[derive(Component, Clone)]
pub struct Draft;

#[derive(Component, Clone)]
pub struct PostDate(pub String);

#[derive(Component, Clone)]
pub struct PostDescription(pub String);

#[derive(Component, Clone)]
pub struct Slug(pub String);

#[derive(Component, Clone, Default)]
pub struct Post;
