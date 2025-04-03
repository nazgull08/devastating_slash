use std::collections::HashSet;

use eframe::{egui, App, Frame};
use egui::{Color32, Pos2, Shape, Stroke};

use legion::{world::SubWorld, *};

/// === ECS COMPONENTS ===
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct HexPos {
    q: i32,
    r: i32,
}

#[derive(Copy, Clone, Debug)]
struct Player;

#[derive(Copy, Clone, Debug)]
struct Movable;

/// === ECS RESOURCES ===
#[derive(Default)]
struct SelectedHex {
    pos: Option<HexPos>,
}

struct Map {
    available: HashSet<HexPos>,
}

/// === MAIN APP ===
#[derive()]
struct MyApp {
    world: World,
    schedule: Schedule,
    resources: Resources,
}

impl MyApp {
    fn new() -> Self {
        let mut world = World::default();

        world.push((
            HexPos { q: 0, r: 0 },
            Player,
            Movable,
        ));

        let mut resources = Resources::default();

        // Примитивная карта 5x5
        let mut available = HashSet::new();
        for q in -2..=2 {
            for r in -2..=2 {
                available.insert(HexPos { q, r });
            }
        }

        resources.insert(Map { available });
        resources.insert(SelectedHex::default());

        let schedule = Schedule::builder()
            .add_system(player_movement_system())
            .build();

        Self { world, schedule, resources }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let hex_size = 30.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            // обработка курсора
            if let Some(mouse_pos) = ctx.input(|i| i.pointer.primary_clicked()).then(|| {
                ctx.input(|i| i.pointer.hover_pos())
            }).flatten() {
                // преобразуем позицию в HexPos
                let clicked_hex = pixel_to_hex(mouse_pos, hex_size);
                self.resources.get_mut::<SelectedHex>().unwrap().pos = Some(clicked_hex);
            }

            // отрисовка карты
            let map = self.resources.get::<Map>().unwrap();
            for pos in &map.available {
                let center = hex_to_pixel(*pos, hex_size);
                draw_hex(&painter, center, hex_size, Color32::from_rgb(100, 200, 255));
            }

            // отрисовка юнита
            let mut query = <(&HexPos, &Player)>::query();
            for (pos, _) in query.iter(&self.world) {
                let center = hex_to_pixel(*pos, hex_size);
                painter.circle_filled(center, 10.0, Color32::LIGHT_GREEN);
            }
        });

        // выполняем системы
        self.schedule.execute(&mut self.world, &mut self.resources);
    }

}

/// === SYSTEMS ===
#[system]
fn player_movement(
    world: &mut SubWorld,
    #[resource] selected: &mut SelectedHex,
    #[resource] map: &Map,
    query: &mut Query<(&mut HexPos, &Player, &Movable)>,
) {
    if let Some(target) = selected.pos.take() {
        if map.available.contains(&target) {
            for (pos, _, _) in query.iter_mut(world) {
                *pos = target;
            }
        }
    }
}

/// === HEX UTILS ===

fn hex_to_pixel(pos: HexPos, size: f32) -> Pos2 {
    let x = size * (3f32.sqrt() * pos.q as f32 + (3f32.sqrt() / 2.0) * pos.r as f32);
    let y = size * (1.5 * pos.r as f32);
    Pos2::new(x + 400.0, y + 300.0)
}

fn pixel_to_hex(pos: Pos2, size: f32) -> HexPos {
    let px = pos.x - 400.0;
    let py = pos.y - 300.0;

    let q = (3f32.sqrt() / 3.0 * px - 1.0 / 3.0 * py) / size;
    let r = (2.0 / 3.0 * py) / size;

    axial_round(q, r)
}

fn axial_round(qf: f32, rf: f32) -> HexPos {
    let mut q = qf.round();
    let mut r = rf.round();
    let s = -q - r;

    let dq = (q - qf).abs();
    let dr = (r - rf).abs();
    let ds = (s + qf + rf).abs();

    if dq > dr && dq > ds {
        q = -r - s;
    } else if dr > ds {
        r = -q - s;
    }

    HexPos { q: q as i32, r: r as i32 }
}

fn draw_hex(painter: &egui::Painter, center: Pos2, radius: f32, color: Color32) {
    let mut points = vec![];
    for i in 0..6 {
        let angle = std::f32::consts::PI / 3.0 * (i as f32 + 0.5);
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Pos2::new(x, y));
    }
    points.push(points[0]);
    painter.add(Shape::line(points, Stroke::new(1.0, color)));
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Devastating Slash",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}
