use bevy::prelude::*;
use bevy_egui::{*, egui::{CursorIcon, Id, InnerResponse, LayerId, Order, Pos2, ScrollArea, Sense, Shape, Ui, color}};

const SPRITE_ID: u64 = 0;

pub fn drag_source(ui: &mut Ui, id: Id, body: impl FnOnce(&mut Ui)) {
    let is_being_dragged = ui.memory().is_being_dragged(id);

    if !is_being_dragged {
        let response = ui.wrap(body).response;

        // Check for drags:
        let response = ui.interact(response.rect, id, Sense::drag());
        if response.hovered() {
            ui.output().cursor_icon = CursorIcon::Default;
        }
    } else {
        ui.output().cursor_icon = CursorIcon::None;

        // Paint the body to a new layer:
        let layer_id = LayerId::new(Order::Tooltip, id);
        let response = ui.with_layer_id(layer_id, body).response;

        // Now we move the visuals of the body to where the mouse is.
        // Normally you need to decide a location for a widget first,
        // because otherwise that widget cannot interact with the mouse.
        // However, a dragged component cannot be interacted with anyway
        // (anything with `Order::Tooltip` always gets an empty `Response`)
        // So this is fine!

        if let Some(pointer_pos) = ui.input().pointer.interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().translate_layer(layer_id, delta);
        }
    }
}

pub fn add_margin(
    ui: &mut Ui,
    margin: [f32; 2],
    body: impl FnOnce(&mut Ui) -> (),
) {
    let margin: bevy_egui::egui::Vec2 = margin.into();
    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds.shrink2(margin);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    body(&mut content_ui);
    let outer_rect = bevy_egui::egui::Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
    ui.allocate_at_least(outer_rect.size(), Sense::hover());
}

pub fn drop_target<R>(
    ui: &mut Ui,
    can_accept_what_is_being_dragged: bool,
    body: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let is_being_dragged = ui.memory().is_anything_being_dragged();

    let margin = bevy_egui::egui::Vec2::new(4.0, 4.0);

    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds.shrink2(margin);
    let where_to_put_background = ui.painter().add(Shape::Noop);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    let ret = body(&mut content_ui);
    let outer_rect = bevy_egui::egui::Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
    let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());

    let style = if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
        ui.visuals().widgets.active
    } else {
        ui.visuals().widgets.inactive
    };

    let mut fill = style.bg_fill;
    let mut stroke = style.bg_stroke;
    if is_being_dragged && !can_accept_what_is_being_dragged {
        // gray out:
        fill = color::tint_color_towards(fill, ui.visuals().window_fill());
        stroke.color = color::tint_color_towards(stroke.color, ui.visuals().window_fill());
    }

    ui.painter().set(
        where_to_put_background,
        Shape::Rect {
            corner_radius: style.corner_radius,
            fill,
            stroke,
            rect,
        },
    );

    InnerResponse::new(ret, response)
}

fn ui(
    ui_context: ResMut<EguiContext>
) {
    egui::Window::new("Hello")
    .resizable(true)
    .show(ui_context.ctx(), |ui| {
        ui.label("world");

        let mut scroll_area = ScrollArea::auto_sized();
        scroll_area.show(ui, |ui| {
            let max_columns = 6;
            for row_index in 0..max_columns {
                ui.columns(max_columns, |columns| {
                    for column_index in 0..max_columns {
                        let ui = &mut columns[column_index];
                        ui.set_max_size([72.0, 72.0].into());
                        ui.set_min_size([72.0, 72.0].into());
                        add_margin(ui, [4.0, 4.0], |ui| {
                            ui.set_max_size([68.0, 68.0].into());
                            ui.set_min_size([68.0, 68.0].into());
                            drop_target(ui, true, |ui| {
                                drag_source(ui, Id::new("item").with(row_index).with(column_index), |ui| {
                                    ui.set_min_size([64.0, 64.0].into());
                                    ui.vertical_centered(|ui| {
                                        ui.centered_and_justified(|ui| {
                                            ui.image(egui::TextureId::User(SPRITE_ID), [64.0, 64.0]);
                                        });
                                    });
                                    
                                });
                            });
                        });
                    }
                });
                ui.end_row();
            }
        });
    });
    egui::Area::new("my_area")
        .fixed_pos(Pos2::new(0.0, 0.0))
        .movable(false)
        .show(ui_context.ctx(), |ui| {
            ui.label("TEST");
        });
}

fn startup(
    asset_server: ResMut<AssetServer>,
    mut ui_context: ResMut<EguiContext>
) {
    let texture_handle = asset_server.load("textures/health_potion.png");
    ui_context.set_egui_texture(SPRITE_ID, texture_handle);    
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("ui-prototype"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(startup.system())
        .add_system(ui.system())
        .run();
}
