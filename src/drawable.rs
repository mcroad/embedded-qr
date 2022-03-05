use crate::{DataTooLong, QrCode};
use core::{marker::PhantomData, mem::size_of};
use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{PixelColor, Point, RgbColor},
    Drawable, Pixel,
};

pub struct QrDrawable<'a, 'b, C>
where
    C: PixelColor + RgbColor,
{
    qr: &'a QrCode<'a>,
    buff: &'b mut [bool],
    pixel_type: PhantomData<C>,
}

impl<'a, 'b, C> QrDrawable<'a, 'b, C>
where
    C: PixelColor + RgbColor,
{
    pub fn new(qr: &'a QrCode, buff: &'b mut [bool]) -> Self {
        return Self {
            qr,
            buff,
            pixel_type: PhantomData,
        };
    }

    pub fn prepare(&mut self, width: usize) -> Result<(), DataTooLong> {
        if width >= 2usize.pow((size_of::<usize>() * 4) as u32) {
            // error
            return Err(DataTooLong::SegmentTooLong);
        }

        let margin_size = 1;
        let s = self.qr.size();
        let data_length = s as usize;
        let data_length_with_margin = data_length + 2 * margin_size;
        let point_size = width / data_length_with_margin;
        if point_size == 0 {
            // error
            return Err(DataTooLong::SegmentTooLong);
        }

        let margin = (width - (point_size * data_length)) / 2;

        // let mut buff = [false; HEIGHT * HEIGHT];
        for i in 0..s {
            for j in 0..s {
                if self.qr.get_module(i, j) {
                    let x = i as usize * point_size + margin;
                    let y = j as usize * point_size + margin;

                    for j in y..(y + point_size) {
                        let offset = j * width;
                        for i in x..(x + point_size) {
                            // buff[offset + i] = true;
                            self.buff[offset + i] = true;
                        }
                    }
                }
            }
        }

        return Ok(());
    }
}

impl<C> Drawable for QrDrawable<'_, '_, C>
where
    C: PixelColor + RgbColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let center = target.bounding_box().center();

        let pixels = self.buff.iter().enumerate().map(|(i, is_black)| {
            let i: i32 = i as i32;
            let y = i / 240;
            let x = i - (y * 240);
            let color = if *is_black { C::BLACK } else { C::WHITE };
            let x = x + (center.x / 4);
            Pixel(Point::new(x, y), color)
        });

        target.draw_iter(pixels)
    }
}
