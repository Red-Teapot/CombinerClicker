use bevy::prelude::*;
use bevy_tweening::Lens;
use kayak_ui::prelude::*;

pub struct KOffsetLens {
    pub start: Edge<Units>,
    pub end: Edge<Units>,
}

impl Lens<KStyle> for KOffsetLens {
    fn lerp(&mut self, target: &mut KStyle, ratio: f32) {
        let Edge {
            top: start_top,
            bottom: start_bottom,
            left: start_left,
            right: start_right,
        } = self.start;
        let Edge {
            top: end_top,
            bottom: end_bottom,
            left: end_left,
            right: end_right,
        } = self.end;

        let mut offset = Edge::all(Units::Auto);

        let sides = [
            (start_top, end_top, &mut offset.top),
            (start_bottom, end_bottom, &mut offset.bottom),
            (start_left, end_left, &mut offset.left),
            (start_right, end_right, &mut offset.right),
        ];

        for (start, end, target_field) in sides {
            match (start, end) {
                (Units::Percentage(start), Units::Percentage(end)) => {
                    *target_field = Units::Percentage(start + (end - start) * ratio)
                }

                (Units::Pixels(start), Units::Pixels(end)) => {
                    *target_field = Units::Pixels(start + (end - start) * ratio)
                }

                (Units::Stretch(start), Units::Stretch(end)) => {
                    *target_field = Units::Stretch(start + (end - start) * ratio)
                }

                (Units::Auto, Units::Auto) => *target_field = Units::Auto,

                _ => panic!(),
            }
        }

        target.offset = StyleProp::Value(offset);
    }
}

pub struct KBackgroundColorLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<KStyle> for KBackgroundColorLens {
    fn lerp(&mut self, target: &mut KStyle, ratio: f32) {
        target.background_color = StyleProp::Value(self.start * (1.0 - ratio) + self.end * ratio);
    }
}
