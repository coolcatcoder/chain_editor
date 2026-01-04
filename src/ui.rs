use std::ops::{Deref, DerefMut};

use bevy::{
    ecs::system::{IntoObserverSystem, SystemParam},
    feathers::{
        controls::{ButtonProps, button, radio},
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui::Checked,
    ui_widgets::{RadioGroup, ValueChange, observe},
};

#[derive(SystemParam)]
pub struct UiBuilder<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn on_camera<'a>(&'a mut self, camera: Entity) -> Ui<'a, 'w, 's> {
        let deepest_child = self
            .commands
            .spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Start,
                padding: UiRect::all(px(8)),
                row_gap: px(8),
                width: percent(100),
                min_width: px(200),
                ..default()
            })
            .id();
        self.commands
            .spawn((
                UiTargetCamera(camera),
                Node {
                    width: percent(30),
                    height: percent(100),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(10),
                    ..default()
                },
                TabGroup::default(),
                ThemeBackgroundColor(tokens::WINDOW_BG),
            ))
            .add_child(deepest_child);

        Ui {
            ui: deepest_child,
            row: Row {
                entity: Entity::PLACEHOLDER,
                previous_widget: None,
                commands: &mut self.commands,
            },
        }
    }
}

pub struct Ui<'c, 'w, 's> {
    ui: Entity,
    row: Row<'c, 'w, 's>,
}

impl Ui<'_, '_, '_> {
    pub fn radio_buttons<const LENGTH: usize, T: Resource + Clone + Component>(
        &mut self,
        radio_buttons: [(&'static str, T); LENGTH],
    ) {
        // Set default. See below comment.
        self.row.commands.insert_resource(radio_buttons[0].1.clone());

        let buttons = radio_buttons.map(|(text, value)| {
            self.row
                .commands
                .spawn((value, radio((), Spawn((Text::new(text), ThemedText)))))
                .id()
        });
        // The first one can be the default.
        // I might change this later to be an explicit index as a parameter.
        self.row.commands.entity(buttons[0]).insert(Checked);

        let child = self
            .row
            .commands
            .spawn((
                RadioGroup,
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(4),
                    ..default()
                },
                observe(move |on: On<ValueChange<Entity>>, mut value: ResMut<T>, values: Query<&T>, mut commands: Commands| {
                    for button in buttons {
                        commands.entity(button).remove::<Checked>();
                    }
                    commands.entity(on.value).insert(Checked);

                    *value = values.get(on.value).expect("All radio buttons will have T.").clone();
                }),
            ))
            .id();

        for button in buttons {
            self.row.commands.entity(child).add_child(button);
        }

        self.row.commands.entity(self.ui).add_child(child);
    }
}

pub struct Row<'c, 'w, 's> {
    entity: Entity,
    previous_widget: Option<Entity>,
    commands: &'c mut Commands<'w, 's>,
}

impl<'c, 'w, 's> Deref for Ui<'c, 'w, 's> {
    type Target = Row<'c, 'w, 's>;

    fn deref(&self) -> &Self::Target {
        &self.row
    }
}

impl<'c, 'w, 's> DerefMut for Ui<'c, 'w, 's> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        info!("Spawned row.");

        let row = self
            .row
            .commands
            .spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                column_gap: px(8),
                ..default()
            })
            .id();

        self.row.commands.entity(self.ui).add_child(row);

        self.row.entity = row;
        &mut self.row
    }
}

impl Row<'_, '_, '_> {
    pub fn observe<E: EntityEvent, B: Bundle, M>(
        &mut self,
        observer: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self {
        if let Some(previous_widget) = self.previous_widget {
            self.commands.entity(previous_widget).observe(observer);
        } else {
            warn!("There is no previous widget to observe.");
        }
        self
    }

    pub fn button(&mut self, text: impl Into<String>) -> &mut Self {
        let button = self
            .commands
            .spawn(button(
                ButtonProps::default(),
                (),
                Spawn((Text::new(text), ThemedText)),
            ))
            .id();
        self.commands.entity(self.entity).add_child(button);
        self.previous_widget = Some(button);
        self
    }
}
