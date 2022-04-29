use bevy::ecs::component::Component;
use bevy::prelude::Vec2;

#[derive(Component, Copy, Clone)]
pub enum ColliderType {
    Circle(f32),
    Rectangle(Vec2),
}

impl ColliderType {
    pub fn is_colliding(&self, self_position: &Vec2, other_collider: &ColliderType, other_position: &Vec2) -> bool {
        match self {
            ColliderType::Circle(self_radius) => {
                match other_collider {
                    ColliderType::Circle(other_radius) => circle_circle_collision(&self_position, *self_radius, &other_position, *other_radius),
                    ColliderType::Rectangle(other_size) => circle_rectangle_collision(&self_position, *self_radius, &other_position, other_size),
                }
            }
            ColliderType::Rectangle(self_size) => {
                match other_collider {
                    ColliderType::Circle(other_radius) => circle_rectangle_collision(&other_position, *other_radius, &self_position, self_size),
                    ColliderType::Rectangle(other_size) => rectangle_rectangle_collision(&self_position, self_size, &other_position, other_size),
                }
            }
        }
    }

    pub fn get_collision_resolution_position(&self, self_position: &Vec2, other_collider: &ColliderType, other_position: &Vec2) -> Vec2 {
        match self {
            ColliderType::Circle(self_radius) => {
                match other_collider {
                    ColliderType::Circle(other_radius) => {
                        let direction = (*other_position - *self_position).normalize_or_zero();

                        *other_position + (-direction * (self_radius + other_radius))
                    }

                    ColliderType::Rectangle(other_size) => {
                        let rectangle_min = *other_position - (*other_size * 0.5);

                        let nearest_point = Vec2::new(
                            f32::max(rectangle_min.x, f32::min(self_position.x, rectangle_min.x + other_size.x)),
                            f32::max(rectangle_min.y, f32::min(self_position.y, rectangle_min.y + other_size.y)),
                        );

                        let nearest_point_distance = nearest_point.distance(*self_position);
                        let ray = nearest_point - *self_position;
                        let overlap = self_radius - nearest_point_distance;

                        *self_position - (ray.normalize_or_zero() * overlap)
                    }
                }
            }

            ColliderType::Rectangle(self_size) => {
                match other_collider {
                    ColliderType::Circle(other_radius) => {
                        let rectangle_min = *self_position - (*self_size * 0.5);

                        let nearest_point = Vec2::new(
                            f32::max(rectangle_min.x, f32::min(other_position.x, rectangle_min.x + self_size.x)),
                            f32::max(rectangle_min.y, f32::min(other_position.y, rectangle_min.y + self_size.y)),
                        );

                        let nearest_point_distance = nearest_point.distance(*other_position);
                        let ray = nearest_point - *other_position;
                        let overlap = other_radius - nearest_point_distance;

                        *self_position + (ray.normalize_or_zero() * (overlap))
                    }

                    ColliderType::Rectangle(other_size) => {
                        let direction = *self_position - *other_position;

                        if (direction.x / (other_size.x)).abs() > (direction.y / (other_size.y)).abs() {
                            if direction.x < 0.0 {
                                Vec2::new(other_position.x - other_size.x, self_position.y)
                            } else {
                                Vec2::new(other_position.x + other_size.x, self_position.y)
                            }
                        } else if direction.y < 0.0 {
                            Vec2::new(self_position.x, other_position.y - other_size.y)
                        } else {
                            Vec2::new(self_position.x, other_position.y + other_size.y)
                        }
                    }
                }
            }
        }
    }
}


fn circle_circle_collision(
    a_circle_position: &Vec2,
    a_circle_radius: f32,
    b_circle_position: &Vec2,
    b_circle_radius: f32,
) -> bool {
    let distance = a_circle_position.distance(*b_circle_position);

    return distance < a_circle_radius + b_circle_radius;
}

fn circle_rectangle_collision(
    circle_position: &Vec2,
    circle_radius: f32,
    rectangle_position: &Vec2,
    rectangle_size: &Vec2,
) -> bool {
    let rectangle_min = *rectangle_position - (*rectangle_size * 0.5);

    let nearest_point = Vec2::new(
        f32::max(rectangle_min.x, f32::min(circle_position.x, rectangle_min.x + rectangle_size.x)),
        f32::max(rectangle_min.y, f32::min(circle_position.y, rectangle_min.y + rectangle_size.y)),
    );

    let ray = nearest_point - *circle_position;
    let mut overlap = circle_radius - ray.length();

    if overlap.is_nan() { overlap = 0.0; }

    overlap > 0.0
}

fn rectangle_rectangle_collision(
    a_rectangle_position: &Vec2,
    a_rectangle_size: &Vec2,
    b_rectangle_position: &Vec2,
    b_rectangle_size: &Vec2,
) -> bool {
    let a_min = *a_rectangle_position - *a_rectangle_size / 2.0;
    let a_max = *a_rectangle_position + *a_rectangle_size / 2.0;

    let b_min = *b_rectangle_position - *b_rectangle_size / 2.0;
    let b_max = *b_rectangle_position + *b_rectangle_size / 2.0;

    a_min.x <= b_max.x
        && a_max.x >= b_min.x
        && a_min.y <= b_max.y
        && a_max.y >= b_min.y
}