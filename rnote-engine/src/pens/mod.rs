pub mod brush;
pub mod eraser;
pub mod penbehaviour;
pub mod selector;
pub mod shaper;
pub mod tools;

use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

use crate::render::Renderer;
use crate::sheet::Sheet;
use crate::strokes::inputdata::InputData;

use self::penbehaviour::PenBehaviour;
use self::tools::Tools;
use self::{brush::Brush, eraser::Eraser, selector::Selector, shaper::Shaper};
use gtk4::{glib, Snapshot};
use p2d::bounding_volume::AABB;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug, glib::Enum, Serialize, Deserialize)]
#[repr(u32)]
#[enum_type(name = "PenStyle")]
#[serde(rename = "pen_style")]
pub enum PenStyle {
    #[enum_value(name = "BrushStyle", nick = "brush_style")]
    #[serde(rename = "brush_style")]
    BrushStyle,
    #[enum_value(name = "ShaperStyle", nick = "shaper_style")]
    #[serde(rename = "shaper_style")]
    ShaperStyle,
    #[enum_value(name = "EraserStyle", nick = "eraser_style")]
    #[serde(rename = "eraser_style")]
    EraserStyle,
    #[enum_value(name = "SelectorStyle", nick = "selector_style")]
    #[serde(rename = "selector_style")]
    SelectorStyle,
    #[enum_value(name = "ToolsStyle", nick = "tools_style")]
    #[serde(rename = "tools_style")]
    ToolsStyle,
}

impl Default for PenStyle {
    fn default() -> Self {
        Self::BrushStyle
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(default, rename = "pens")]
pub struct Pens {
    #[serde(rename = "style")]
    pub style: PenStyle,
    #[serde(rename = "style_overwrite")]
    pub style_overwrite: Option<PenStyle>,

    #[serde(rename = "brush")]
    pub brush: Brush,
    #[serde(rename = "shaper")]
    pub shaper: Shaper,
    #[serde(rename = "eraser")]
    pub eraser: Eraser,
    #[serde(rename = "selector")]
    pub selector: Selector,
    #[serde(rename = "tools")]
    pub tools: Tools,

    #[serde(skip)]
    pen_shown: bool,
}

impl PenBehaviour for Pens {
    fn begin(
        &mut self,
        data_entries: VecDeque<InputData>,
        sheet: &mut crate::sheet::Sheet,
        viewport: Option<AABB>,
        zoom: f64,
        renderer: Arc<RwLock<Renderer>>,
    ) {
        self.pen_shown = true;

        match self.current_style() {
            PenStyle::BrushStyle => {
                self.brush
                    .begin(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::ShaperStyle => {
                self.shaper
                    .begin(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::EraserStyle => {
                self.eraser
                    .begin(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::SelectorStyle => {
                self.selector
                    .begin(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::ToolsStyle => {
                self.tools
                    .begin(data_entries, sheet, viewport, zoom, renderer);
            }
        }
    }

    fn motion(
        &mut self,
        data_entries: VecDeque<InputData>,
        sheet: &mut crate::sheet::Sheet,
        viewport: Option<AABB>,
        zoom: f64,
        renderer: Arc<RwLock<Renderer>>,
    ) {
        match self.current_style() {
            PenStyle::BrushStyle => {
                self.brush
                    .motion(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::ShaperStyle => {
                self.shaper
                    .motion(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::EraserStyle => {
                self.eraser
                    .motion(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::SelectorStyle => {
                self.selector
                    .motion(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::ToolsStyle => {
                self.tools
                    .motion(data_entries, sheet, viewport, zoom, renderer);
            }
        }
    }

    fn end(
        &mut self,
        data_entries: VecDeque<InputData>,
        sheet: &mut crate::sheet::Sheet,
        viewport: Option<AABB>,
        zoom: f64,
        renderer: Arc<RwLock<Renderer>>,
    ) {
        match self.current_style() {
            PenStyle::BrushStyle => {
                self.brush
                    .end(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::ShaperStyle => {
                self.shaper
                    .end(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::EraserStyle => {
                self.eraser
                    .end(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::SelectorStyle => {
                self.selector
                    .end(data_entries, sheet, viewport, zoom, renderer);
            }
            PenStyle::ToolsStyle => {
                self.tools
                    .end(data_entries, sheet, viewport, zoom, renderer);
            }
        }

        self.pen_shown = false;
        self.style_overwrite = None;
    }

    fn draw(
        &self,
        snapshot: &Snapshot,
        sheet: &Sheet,
        viewport: Option<AABB>,
        zoom: f64,
        renderer: Arc<RwLock<Renderer>>,
    ) -> Result<(), anyhow::Error> {
        if self.pen_shown {
            match self.current_style() {
                PenStyle::BrushStyle => self.brush.draw(snapshot, sheet, viewport, zoom, renderer),
                PenStyle::ShaperStyle => {
                    self.shaper.draw(snapshot, sheet, viewport, zoom, renderer)
                }
                PenStyle::EraserStyle => {
                    self.eraser.draw(snapshot, sheet, viewport, zoom, renderer)
                }
                PenStyle::SelectorStyle => self
                    .selector
                    .draw(snapshot, sheet, viewport, zoom, renderer),
                PenStyle::ToolsStyle => self.tools.draw(snapshot, sheet, viewport, zoom, renderer),
            }
        } else {
            Ok(())
        }
    }
}

impl Pens {
    pub fn pen_shown(&self) -> bool {
        self.pen_shown
    }

    pub fn current_style(&self) -> PenStyle {
        self.style_overwrite.unwrap_or(self.style)
    }
}