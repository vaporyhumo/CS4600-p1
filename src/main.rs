#[derive(Clone, Copy, Debug)]
struct RGBA {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

#[derive(Clone, Debug)]
struct Image {
  width: u32,
  height: u32,
  pixels: Vec<RGBA>,
}

#[derive(Clone, Copy, Debug)]
struct ImagePixel {
  x: u32,
  y: u32,
  color: RGBA,
}

impl Image {
  fn pixel_at(&self, x: i32, y: i32) -> Option<ImagePixel> {
    if x < 0 || y < 0 {
      return None;
    }
    let x = x as u32;
    let y = y as u32;
    if x < self.width && y < self.height {
      let index = (y * self.width + x) as usize;
      Some(ImagePixel { x, y, color: self.pixels[index] })
    } else {
      None
    }
  }

  fn index_to_coords(&self, index: usize) -> (u32, u32) {
    let x = index % self.width as usize;
    let y = index / self.width as usize;
    (x as u32, y as u32)
  }

  fn pixels(&self) -> Vec<ImagePixel> {
    let mut pixels = Vec::new();
    for index in 0..self.pixels.len() {
      let (x, y) = self.index_to_coords(index);
      pixels.push(ImagePixel { x, y, color: self.pixels[index] });
    }
    pixels
  }
}

fn alpha_blend_mut(background: &mut Image, foreground: Image, offset: (i32, i32), alpha: f32) -> () {
  *background = alpha_blend(background.clone(), foreground, offset, alpha);
}

fn alpha_blend(background: Image, foreground: Image, offset: (i32, i32), alpha: f32) -> Image {
  let mut background_pixels: Vec<ImagePixel> = background.pixels();

  background_pixels = background_pixels
    .iter()
    .cloned()
    .map(|pixel| {
      let x = pixel.x as i32 - offset.0;
      let y = pixel.y as i32 - offset.1;
      let foreground_pixel = foreground.pixel_at(x, y);
      match foreground_pixel {
        Some(foreground_pixel) => blend_pixels(pixel, alpha, foreground_pixel),
        None => pixel.clone(),
      }
    })
    .collect();
  Image {
    width: background.width,
    height: background.height,
    pixels: background_pixels.iter().map(|pixel| pixel.color).collect(),
  }
}

fn blend_pixels(pixel: ImagePixel, alpha: f32, foreground_pixel: ImagePixel) -> ImagePixel {
  let r: u8 =
    (pixel.color.r as f32 * (1.0 - alpha) + foreground_pixel.color.r as f32 * alpha) as u8;
  let g: u8 =
    (pixel.color.g as f32 * (1.0 - alpha) + foreground_pixel.color.g as f32 * alpha) as u8;
  let b: u8 =
    (pixel.color.b as f32 * (1.0 - alpha) + foreground_pixel.color.b as f32 * alpha) as u8;
  let a: u8 =
    (pixel.color.a as f32 * (1.0 - alpha) + foreground_pixel.color.a as f32 * alpha) as u8;
  ImagePixel { x: pixel.x, y: pixel.y, color: RGBA { r, g, b, a } }
}

fn main() {
  let mut background: Image = Image {
    width: 2,
    height: 2,
    pixels: vec![
      RGBA { r: 0, g: 0, b: 0, a: 255 },
      RGBA { r: 255, g: 0, b: 0, a: 255 },
      RGBA { r: 0, g: 255, b: 0, a: 255 },
      RGBA { r: 0, g: 0, b: 255, a: 255 },
    ],
  };
  let foreground: Image =
    Image { width: 1, height: 1, pixels: vec![RGBA { r: 255, g: 255, b: 255, a: 255 }] };
  let alpha: f32 = 0.5;
  let offset: (i32, i32) = (1, 0);

  alpha_blend_mut(&mut background, foreground, offset, alpha);
  println!("{:#?}", background);
}
