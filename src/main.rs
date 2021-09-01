#![allow(unused_imports)]
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use raylib4_sys::*;
use std::ffi::c_void;
use std::ffi::CString;
use std::path::Path;
use std::process::Termination;

use colors::*;
use texture::*;

pub mod colors;
pub mod texture;

fn rand() -> f32 {
    let step = Uniform::new(-3.40, 3.40); // f32::MIN causing overflow ??
    let mut rng = rand::thread_rng();
    step.sample(&mut rng)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let factor: f32 = 100_f32;
    let screen_width: f32 = 16_f32 * factor;
    let screen_height: f32 = 8_f32 * factor;
    let scalar: f32 = 1_f32;
    let texture_width = screen_width * scalar;
    let texture_height: f32 = screen_height * scalar;

    unsafe {
        InitWindow(1200, 800, "title".as_ptr() as *const i8);
        SetTargetFPS(15);

        let image = GenImageCellular(
            texture_width as i32,
            texture_height as i32,
            (texture_height / 6_f32) as i32,
        );

        // let mut image = GenImageColor(texture_width as i32, texture_height as i32, BLACK);

        // for y in 0..(texture_height) as i32 {
        //     for x in 0..(texture_width) as i32 {
        //         let v: u8 = (rand() * 255.0_f32) as u8;
        //         println!("v: {}", v);
        //         let color: Color = Color {
        //             r: v,
        //             g: v,
        //             b: v,
        //             a: 255,
        //         };

        //         ImageDrawPixel(&mut image, x, y, color);
        //     }
        // }

        let mut state: [RenderTexture2D; 2] = [std::mem::zeroed(), std::mem::zeroed()];

        state[0] = LoadRenderTexture(texture_width as i32, texture_height as i32);
        SetTextureWrap(state[0].texture, TextureWrap_TEXTURE_WRAP_REPEAT as i32);
        SetTextureFilter(
            state[0].texture,
            TextureFilter_TEXTURE_FILTER_BILINEAR as i32,
        );
        UpdateTexture(state[0].texture, image.data);

        state[1] = LoadRenderTexture(texture_width as i32, texture_height as i32);
        SetTextureWrap(state[1].texture, TextureWrap_TEXTURE_WRAP_REPEAT as i32);
        SetTextureFilter(
            state[1].texture,
            TextureFilter_TEXTURE_FILTER_BILINEAR as i32,
        );
        // check "../../smoothlife.fs" exists

        if !Path::exists(Path::new("smoothlife.fs")) {
            return Err("File not found".into());
        }

        let f_n = CString::new("smoothlife.fs").unwrap();
        let f_nptr = f_n.as_ptr();

        let shader: Shader = LoadShader(std::ptr::null() as *const i8, f_nptr as *const i8);

        let resolution: Vector2 = Vector2 {
            x: texture_width,
            y: texture_height,
        };

        let res_n = CString::new("resolution").unwrap();
        let res_nptr = res_n.as_ptr();

        let resolution_loc: i32 = GetShaderLocation(shader, res_nptr as *const i8);
        SetShaderValue(
            shader,
            resolution_loc,
            &resolution as *const Vector2 as *const std::ffi::c_void,
            ShaderUniformDataType_SHADER_UNIFORM_VEC2 as i32,
        );
        let mut i: usize = 0;

        while !WindowShouldClose() {
            BeginTextureMode(state[1 - i]);
            ClearBackground(BLACK);
            BeginShaderMode(shader);
            DrawTexture(state[i].texture, 0, 0, WHITE);
            EndShaderMode();
            EndTextureMode();

            i = 1 - i;

            BeginDrawing();
            ClearBackground(BLACK);
            DrawTextureEx(
                state[i].texture,
                Vector2 { x: 0_f32, y: 0_f32 },
                0_f32,
                1_f32 / scalar,
                WHITE,
            );
            // DrawTexture(state[i].texture, 0, 0, WHITE);
            EndDrawing();
        }

        CloseWindow();

        // while !raylib4_sys::WindowShouldClose() {
        //     BeginDrawing();
        //     ClearBackground(RAYWHITE);
        //     DrawText(
        //         format!(
        //             "Congrats! You created your first window! ur lucky number :{}",
        //             rand()
        //         )
        //         .as_ptr() as *const i8,
        //         50,
        //         200,
        //         20,
        //         LIGHTGRAY,
        //     );
        //     EndDrawing();
        // }
        CloseWindow()
    }
    Ok(())
}
