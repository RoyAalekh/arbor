use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::{ImageBuffer, Rgb};

pub fn export_as_png(tree_text: &str, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new image with white background
    let width = 800u32;
    let height = (tree_text.lines().count() as u32) * 15 + 40;
    let mut image = ImageBuffer::from_fn(width, height, |_, _| Rgb([255u8, 255u8, 255u8]));

    // Simple text rendering using basic shapes
    for (i, line) in tree_text.lines().enumerate() {
        let y_pos = (i as u32 * 15) + 10;
        let chars = line.chars();
        let mut x_pos = 10u32;

        for c in chars {
            // Draw each character as a simple pixel pattern
            match c {
                '│' => draw_vertical_line(&mut image, x_pos, y_pos),
                '├' => draw_t_shape(&mut image, x_pos, y_pos),
                '└' => draw_l_shape(&mut image, x_pos, y_pos),
                '─' => draw_horizontal_line(&mut image, x_pos, y_pos),
                _ => draw_text_char(&mut image, x_pos, y_pos, c),
            }
            x_pos += 8;
        }
    }

    image.save(output_path)?;
    Ok(())
}

fn draw_vertical_line(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32) {
    for dy in 0..12 {
        draw_pixel(image, x + 3, y + dy);
    }
}

fn draw_horizontal_line(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32) {
    for dx in 0..8 {
        draw_pixel(image, x + dx, y + 6);
    }
}

fn draw_t_shape(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32) {
    draw_vertical_line(image, x, y);
    for dx in 3..8 {
        draw_pixel(image, x + dx, y + 6);
    }
}

fn draw_l_shape(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32) {
    for dy in 0..7 {
        draw_pixel(image, x + 3, y + dy);
    }
    for dx in 3..8 {
        draw_pixel(image, x + dx, y + 6);
    }
}

fn draw_text_char(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, c: char) {
    let color = match c {
        _ if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() => Rgb([0u8, 0u8, 0u8]),
        _ => Rgb([128u8, 128u8, 128u8]),
    };

    // Draw a simple dot pattern for the character
    for dy in 0..8 {
        for dx in 0..6 {
            if should_draw_pixel(c, dx, dy) {
                draw_pixel_with_color(image, x + dx, y + dy, color);
            }
        }
    }
}

fn should_draw_pixel(c: char, x: u32, y: u32) -> bool {
    // Simple pixel patterns for characters
    match c {
        '.' => x == 2 && y == 6,
        '/' => x == y,
        '\\' => x + y == 7,
        '_' => y == 7,
        '-' => y == 4,
        '|' => x == 2,
        _ => false,
    }
}

fn draw_pixel(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32) {
    draw_pixel_with_color(image, x, y, Rgb([0u8, 0u8, 0u8]));
}

fn draw_pixel_with_color(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, color: Rgb<u8>) {
    if x < image.width() && y < image.height() {
        image.put_pixel(x, y, color);
    }
}

pub fn export_as_mermaid(relationships: &[(String, String)], output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = String::from("graph TD\n");

    for (parent, child) in relationships {
        let parent_id = sanitize_id(parent);
        let child_id = sanitize_id(child);
        output.push_str(&format!("    {}[{}] --> {}[{}]\n", 
            parent_id, parent, child_id, child));
    }

    let mut file = File::create(output_path)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn sanitize_id(s: &str) -> String {
    s.replace([' ', '.', '-'], "_")
}