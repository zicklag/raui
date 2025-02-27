use crate::ui::components::app::{app, AppProps};
use lipsum::MarkovChain;
use raui_core::{prelude::*, widget::setup as setup_core};
use raui_material::setup as setup_material;
use raui_tetra_renderer::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string};
use tetra::{
    graphics::{self, Color, DrawParams, Texture},
    input::{is_key_modifier_down, Key, KeyModifier},
    Context, Event, State,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct AssetsManifest {
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub fonts: HashMap<String, (usize, Scalar, String)>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub images: HashMap<String, String>,
}

fn setup(app: &mut Application) {
    app.setup(setup_core);
    app.setup(setup_material);
}

pub struct AppState {
    ui: TetraSimpleHost,
    mockup_image: Texture,
}

impl AppState {
    pub fn new(context: &mut Context) -> tetra::Result<Self> {
        let mockup_image = Texture::new(context, "./resources/images/in-game-mockup.png")?;
        let assets = serde_json::from_str::<AssetsManifest>(
            &read_to_string("./resources/assets.json").expect("Could not load assets manifest!"),
        )
        .expect("Could not parse assets manifest");
        let fonts = assets
            .fonts
            .iter()
            .map(|(k, (s, f, p))| (k.as_str(), *s, *f, p.as_str()))
            .collect::<Vec<_>>();
        let textures = assets
            .images
            .iter()
            .map(|(k, p)| (k.as_str(), p.as_str()))
            .collect::<Vec<_>>();
        let text = read_to_string("./resources/text.txt").expect("Could not load texts!");
        let mut markov = MarkovChain::new();
        markov.learn(&text);
        let props = AppProps {
            texts: (0..=18).map(|_| markov.generate(20)).collect::<Vec<_>>(),
        };
        let tree = widget! { (#{"app"} app: {props}) };
        let mut ui = TetraSimpleHost::new(context, tree, &fonts, &textures, setup)?;
        const SCALE: Scalar = 256.0;
        ui.scaling = CoordsMappingScaling::FitMinimum(Vec2 { x: SCALE, y: SCALE });
        Ok(Self { ui, mockup_image })
    }
}

impl State for AppState {
    fn update(&mut self, context: &mut Context) -> tetra::Result {
        self.ui.update(context);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> tetra::Result {
        graphics::clear(context, Color::WHITE);
        {
            let width = tetra::window::get_width(context) as Scalar;
            let height = tetra::window::get_height(context) as Scalar;
            let w = self.mockup_image.width() as Scalar;
            let h = self.mockup_image.height() as Scalar;
            let ra = width / height;
            let va = w / h;
            let scale = if ra >= va { width / w } else { height / h };
            let ox = (width - w * scale) * 0.5;
            let oy = (height - h * scale) * 0.5;
            let params = DrawParams::default()
                .position([ox, oy].into())
                .scale([scale, scale].into());
            self.mockup_image.draw(context, params);
        }
        self.ui.draw(context)?;
        Ok(())
    }

    fn event(&mut self, context: &mut Context, event: Event) -> tetra::Result {
        self.ui.event(context, &event);
        if let Event::KeyPressed { key: Key::P } = event {
            if is_key_modifier_down(context, KeyModifier::Ctrl) {
                println!("LAYOUT: {:#?}", self.ui.application.layout_data());
                if is_key_modifier_down(context, KeyModifier::Shift) {
                    println!("INTERACTIONS: {:#?}", self.ui.interactions);
                }
            }
        }
        Ok(())
    }
}
