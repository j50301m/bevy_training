use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_layout_demo)
        .run();
}

fn setup_layout_demo(mut commands: Commands) {
    // Add camera
    commands.spawn(Camera2d);

    // Create root container - using Flexbox layout
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column, // Vertical layout
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    )).with_children(|parent| {
        // Example 1: Relative positioned elements (normal layout flow)
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Row, // Horizontal layout
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
        )).with_children(|container| {
            // These buttons will be automatically arranged according to layout flow
            for i in 1..=3 {
                container.spawn((
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(40.0),
                        position_type: PositionType::Relative, // Participate in layout flow
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                )).with_children(|button| {
                    button.spawn((
                        Text::new(format!("Button{i}")),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });

        // Example 2: Container with Absolute positioned elements
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(100.0),
                position_type: PositionType::Relative,
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
        )).with_children(|container| {
            // Normal text in layout flow
            container.spawn((
                Text::new("This is normal text in the container"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Relative,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Absolutely positioned overlay - outside of layout flow
            container.spawn((
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(30.0),
                    position_type: PositionType::Absolute, // Outside of layout flow
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
            )).with_children(|overlay| {
                overlay.spawn((
                    Text::new("Overlay"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });

        // Example 3: Description text
        parent.spawn((
            Text::new("The \"Overlay\" button in the red container above uses absolute positioning\nIt is outside the layout flow and doesn't affect other elements' positions"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                max_width: Val::Px(400.0),
                ..default()
            },
        ));
    });
}
