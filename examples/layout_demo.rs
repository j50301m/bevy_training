use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_layout_demo)
        .run();
}

fn setup_layout_demo(mut commands: Commands) {
    // 添加相机
    commands.spawn(Camera2d);

    // 创建根容器 - 使用 Flexbox 布局
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column, // 垂直排列
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    )).with_children(|parent| {
        // 示例1: Relative 定位的元素们 (正常布局流)
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Row, // 水平排列
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
        )).with_children(|container| {
            // 这些按钮会按照布局流自动排列
            for i in 1..=3 {
                container.spawn((
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(40.0),
                        position_type: PositionType::Relative, // 参与布局流
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                )).with_children(|button| {
                    button.spawn((
                        Text::new(format!("按钮{}", i)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });

        // 示例2: 包含 Absolute 定位元素的容器
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(100.0),
                position_type: PositionType::Relative,
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
        )).with_children(|container| {
            // 正常布局流中的文字
            container.spawn((
                Text::new("这是容器中的正常文字"),
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

            // 绝对定位的覆盖层 - 脱离布局流
            container.spawn((
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(30.0),
                    position_type: PositionType::Absolute, // 脱离布局流
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
            )).with_children(|overlay| {
                overlay.spawn((
                    Text::new("覆盖"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });

        // 示例3: 说明文字
        parent.spawn((
            Text::new("上方红色容器中的\"覆盖\"按钮使用了绝对定位\n它脱离了布局流，不会影响其他元素的位置"),
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
