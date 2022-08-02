Is there a more idiomatic way to:
1. Represent relationship between two or more entities?
2. Implement common queries without passing extra arguments everywhere?

Here is my code:

```rust
#[derive(Component, Inspectable)]
pub struct Player {
    pub name: String,
    pub is_human: bool,
    pub id: usize,
    pub color: Color,
}

#[derive(Component)]
pub struct OwnedBy {
    pub player_id: usize,
}

fn star_assignment_changed(
    mut query_star: Query<(&mut Sprite, &OwnedBy, &Children), (With<Star>, Changed<OwnedBy>)>,
    player_query: Query<&Player>,
) {
    for (mut sprite, owned_by, children) in query_star.iter_mut() {
        let player = find_player_by_id(owned_by.player_id, &player_query);
        if player.is_none() {
            continue;
        }
        let player = player.unwrap();

        sprite.color = player.color;
    }
}

pub fn find_player_by_id<'a>(
    player_id: usize,
    player_query: &'a Query<&Player>,
) -> Option<&'a Player> {
    for player in player_query.iter() {
        if player.id == player_id {
            return Some(player);
        }
    }
    None
}
```

1. In my example I use OwnedBy to represent relationship, but it requires a lot of extra work (queries) to get the connected entities.
My example is also weakly typed, either usually using a basic type to represent a link.
I suppose I could've used an `Entity` instead, which makes the code slightly better:

```rust
#[derive(Component)]
pub struct OwnedBy {
    pub player: Entity,
}


fn star_assignment_changed(
    mut query_star: Query<(&mut Sprite, &OwnedBy, &Children), (With<Star>, Changed<OwnedBy>)>,
    player_query: Query<&Player>, // still need this?
) {
    for (mut sprite, owned_by, children) in query_star.iter_mut() {
        let player = player_query.get_single(owned_by);
        if player.is_none() {
            continue;
        }
        let player = player.unwrap();

        sprite.color = player.color;
    }
}
```

This makes it better, but it's still just an Entity (no guarantee that it will have the Player component), and still requires this player_query thing.

This brings me to my second point.
2. Is there a way to reduce the amount of parameters?