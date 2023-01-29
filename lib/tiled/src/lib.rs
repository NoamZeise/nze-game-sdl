//! A library for loading maps and tilesets made with the Tiled map editor
//! 
//! Loads the map using a path, and also automatically loads any tilesets.
//! Note: infinite maps and templates are unsupported

pub mod error;

mod tileset;
mod layer;
mod object_group;
mod properties;
mod image_layer;
mod helper;
mod map;

pub use properties::Properties;
pub use layer::{Layer, LayerData, LayerTiles};
pub use object_group::*;
pub use tileset::Tileset;
pub use image_layer::ImageLayer;
pub use map::*;

use helper::*;
use error::TiledError;

pub struct Colour {
    pub r : u8,
    pub g : u8,
    pub b : u8,
    pub a : u8,
}

#[cfg(test)]
mod tiled_tests {
    use super::*;
    #[test]
    fn test_map() {
        let map = Map::new("test-resources/test.tmx").unwrap();
        assert!(map.width == 4);
        assert!(map.height == 4);
        assert!(map.tile_width == 10);
        assert!(map.tile_height == 10);
        assert!(!map.infinite);
        assert!(match map.orientation {
            Orientation::Orthogonal => true,
            _ => false,
        });

        assert!(map.tilesets.len() == 1);
        assert!(map.tilesets[0].first_tile_id == 1);
        assert!(map.tilesets[0].tile_width == 10);
        assert!(map.tilesets[0].tile_height == 10);
        assert!(map.tilesets[0].spacing == 2);
        assert!(map.tilesets[0].margin == 5);
        assert!(map.tilesets[0].tile_count == 4);
        assert!(map.tilesets[0].column_count == 2);
        assert!(map.tilesets[0].image_path == "test-resources/test-tileset.png");
        assert!(map.tilesets[0].image_width == 32);
        assert!(map.tilesets[0].image_height == 32);

        assert!(map.layers.len() == 2);
        assert!(map.layers[0].width == 4);
        assert!(map.layers[0].height == 4);
        assert!(
            map.layers[0].tiles == vec![
                4, 4, 0, 0,
                2, 2, 2, 0,
                2, 2, 2, 0,
                4, 4, 0, 0,
            ]
        );
        assert!(map.layers[1].width == 4);
        assert!(map.layers[1].height == 4);
        assert!(map.layers[1].props.booleans["collidable"] == true);
        assert!(
            map.layers[1].tiles == vec![
                0, 0, 0, 0,
                0, 0, 0, 0,
                1, 1, 1, 1,
                1, 1, 1, 1,
            ]
        );
        assert!(map.layers[1].info.id == 1);
        assert!(map.layers[1].info.name == "Tile Layer 1");
        assert!(map.layers[1].info.opacity == 0.90);
        assert!(map.layers[1].info.tint.g == 0);

        assert!(map.obj_groups.len() == 2);
        assert!(map.obj_groups[0].props.booleans["obj_group"] == true);
        assert!(map.obj_groups[0].objs.len() == 2);
        assert!(map.obj_groups[0].polys.len() == 2);
        assert!(map.obj_groups[0].points.len() == 1);
        assert!(map.obj_groups[0].ellipse.len() == 1);
        assert!(map.obj_groups[0].ellipse[0].rotation == 10.0);
        assert!(map.obj_groups[0].points[0].info.name == "dd");
        assert!(map.obj_groups[0].objs[1].props.integers["num"] == 5);
        assert!(map.obj_groups[0].objs[1].props.booleans["test"] == true);
        assert!(map.obj_groups[0].objs[1].rotation == 343.734);
        assert!(map.obj_groups[0].objs[1].info.type_name == "asd");
        assert!(map.obj_groups[0].objs[0].info.name == "barry");
        assert!(map.obj_groups[0].objs[0].info.visible == false);
        assert!(map.obj_groups[0].objs[1].info.id == 2);
        assert!(map.obj_groups[0].objs[1].rect.x == 4.25998);
        assert!(map.obj_groups[0].objs[1].rect.y == 10.0772);
        assert!(map.obj_groups[0].objs[1].rect.w == 15.1719);
        assert!(map.obj_groups[0].objs[1].rect.h == 18.3066);
        assert!(map.obj_groups[0].objs[0].rect.x == 28.6511);
        assert!(map.obj_groups[0].objs[0].rect.y == 10.658);
        assert!(map.obj_groups[0].objs[0].rect.w == 7.71136);
        assert!(map.obj_groups[0].objs[0].rect.h == 12.7269);
        assert!(map.obj_groups[0].polys[1].obj.rect.x == 7.58597);
        assert!(map.obj_groups[0].polys[1].obj.rect.y == 6.33209);
        assert!(map.obj_groups[0].polys[1].obj.rect.w == 0.0);
        assert!(map.obj_groups[0].polys[1].obj.rect.h == 0.0);
        assert!(map.obj_groups[0].polys[1].obj.props.booleans["open"] == true);
        assert!(map.obj_groups[0].polys[1].closed == false);
        assert!(map.obj_groups[0].polys[1].points.iter()
                .map(|Vec2 {x, y}| {
                    (*x as i32,  *y as i32)
                })
                .collect::<Vec<(i32, i32)>>() ==
                vec![
                    (0, 0),
                    (15, -3),
                    (4, 13),
                    ]
        );
        assert!(map.obj_groups[0].polys[0].obj.rect.x == 9.15332);
        assert!(map.obj_groups[0].polys[0].obj.rect.y == 33.7294);
        assert!(map.obj_groups[0].polys[0].obj.rect.w == 0.0);
        assert!(map.obj_groups[0].polys[0].obj.rect.h == 0.0);
        assert!(map.obj_groups[0].polys[0].closed == true);
        assert!(map.obj_groups[0].polys[0].points.iter()
                .map(|Vec2 {x, y}| {
                    (*x as i32,  *y as i32)
                })
                .collect::<Vec<(i32, i32)>>() ==
                vec![
                    (0, 0),
                    (0, -7),
                    (9, -1),
                    ]
        );
        assert!(map.obj_groups[0].text.len() == 1);
        assert!(map.obj_groups[0].text[0].text == "Hello World");
        assert!(map.obj_groups[0].text[0].font_family == "MS Sans Serif");
        assert!(map.obj_groups[0].text[0].colour.r == 98);
        assert!(map.obj_groups[0].text[0].horizontal_align == TextHorizontalAlign::Justify);
        assert!(map.obj_groups[0].text[0].vertical_align == TextVerticalAlign::Center);
        assert!(map.obj_groups[0].text[0].italic == true);
        assert!(map.obj_groups[0].text[0].bold == true);
        assert!(map.obj_groups[0].text[0].wrap == true);
        assert!(map.obj_groups[0].text[0].pixel_size == 29);

        assert!(map.obj_groups[1].info.id == 5);
        assert!(map.obj_groups[1].info.name == "obj2s");
        assert!(map.obj_groups[1].info.index_draw_order == true);
        assert!(map.obj_groups[1].info.offset.x == 5.05);
        assert!(map.obj_groups[1].info.parallax.x == 1.10);
        assert!(map.obj_groups[1].info.colour.r == 85);
        assert!(map.obj_groups[1].info.colour.a == 10);
        assert!(map.obj_groups[1].info.visible == true);
        assert!(map.obj_groups[1].info.locked == true);
        assert!(map.obj_groups[1].info.tint.g == 115);
        assert!(map.obj_groups[1].info.tint.a == 255);
        assert!(map.obj_groups[1].props.booleans["collidable"] == true);
        assert!(map.obj_groups[1].objs.len() == 2);
        assert!(map.obj_groups[1].polys.len() == 0);
        assert!(map.obj_groups[1].objs[1].info.id == 8);
        assert!(map.obj_groups[1].objs[1].rect.x == 10.0);
        assert!(map.obj_groups[1].objs[1].rect.y == 20.0);
        assert!(map.obj_groups[1].objs[1].rect.w == 20.0);
        assert!(map.obj_groups[1].objs[1].rect.h == 0.0);
        assert!(map.obj_groups[1].objs[0].info.id == 9);
        assert!(map.obj_groups[1].objs[0].rect.x == 0.0);
        assert!(map.obj_groups[1].objs[0].rect.y == 0.0);
        assert!(map.obj_groups[1].objs[0].rect.w == 20.0);
        assert!(map.obj_groups[1].objs[0].rect.h == 10.0);
        assert!(map.obj_groups[1].objs[0].props.booleans["test_coll"] == true);

        assert!(map.img_layers.len() == 1);
        assert!(map.img_layers[0].info.offset.x == 19.247);
        assert!(map.img_layers[0].info.offset.y == -10.3445);
        assert!(map.img_layers[0].image_path == "test-tileset.png");
        assert!(map.img_layers[0].width == 32);
        assert!(map.img_layers[0].height == 32);
        assert!(map.img_layers[0].repeat_x == false);
        assert!(map.img_layers[0].repeat_y == true);
        assert!(map.img_layers[0].info.parallax.x == 2.07);
        assert!(map.img_layers[0].info.parallax.y ==  1.0);
        assert!(map.img_layers[0].props.booleans["img"] == false);
    }
}
