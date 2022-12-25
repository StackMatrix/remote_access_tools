use chrono::Local;
use rayon::prelude::{
  IntoParallelRefMutIterator, 
  IndexedParallelIterator, 
  IntoParallelRefIterator, 
  ParallelIterator
};
use zstd::zstd_safe::CCtx;

use super::screen_capture_state::ScreenCaptureStruct;

/// 开启捕捉屏幕
pub async fn take() {
    println!("Screen state- Fetch... {}", Local::now().to_string());
    
    // 通过创建一个压缩上下文，就可以调用 zstd 库提供的压缩和解压函数，实现数据的压缩和解压。
    let mut cctx = CCtx::create();
    // 屏幕捕捉实例化
    let mut screen_capture = ScreenCaptureStruct::instance();

    // 屏幕信息获取
    let (width, height, _size) = screen_capture.display();
    let pixle = width * height * 3;

    // 创建一个 width * height * 3 的 u8 类型数据的 Vec 容器，并预先分配容量，避免动态扩容带来性能损失。
    // 其用于保存一张图片的像素，若 3 是一个像素点有 RGB 三种颜色值, 4 是有 RGBA 四种颜色值。
    // Vec::with_capacity() 方法是用来指定容器的初始容量，避免容器在存储数据时频繁进行内存分配，从而提升程序的性能。
    let mut src_frame = Vec::<u8>::with_capacity(pixle);
    let mut src_frame2 = Vec::<u8>::with_capacity(pixle);
    // 这个变量是用来存储压缩后的像素
    let mut dst_frame = Vec::<u8>::with_capacity(pixle);

    // 一般我们通过添加新的元素来自动设置 Vec 容器的长度，不需要显式调用 set_len() 函数。
    // 但在此处需要显式设置 Vec 容器的长度，因为要预先给 pixels_frame 分配存储数据的容器的容量，避免动态扩容带来的性能损失。
    unsafe{
      src_frame.set_len(pixle);
      src_frame2.set_len(pixle);
      dst_frame.set_len(pixle);
    }

    // 屏幕捕捉开始
    screen_capture.start(&mut src_frame);
    
    // 压缩捕捉到的帧，cctx.compress 可以用来将一个字节数组压缩成另一个字节数组
    let compress_frame = cctx.compress(&mut dst_frame, &src_frame, 22).unwrap();

    // tcp_stream::send(compress_frame);
    

    loop {
      loop {
          // 截图
          screen_capture.start(&mut src_frame2);
          if src_frame2 == src_frame {
              continue;
          }
          // 做减法
          src_frame.par_iter_mut().zip(src_frame2.par_iter()).for_each(|(d1, d2)|{
              *d1 ^= *d2;
          });
          // 压缩
          unsafe {
            dst_frame.set_len(0);
          }
          let clen = cctx.compress(&mut dst_frame, &src_frame, 22).unwrap();
          // let clen = util::compress(&data1, &mut pres_data);
          // 发送diff
          // encode(clen, &mut header);
          // if let Err(_) = stream.write_all(&header) {
          //     return;
          // }
          // if let Err(_) = stream.write_all(&pres_data) {
          //     return;
          // }
          // util::skip(clen);
          // break;
      }

      // loop {
      //     // 截图
      //     cap.cap(&mut data1);
      //     if data1 == data2 {
      //         continue;
      //     }
      //     // 做减法
      //     data2.par_iter_mut().zip(data1.par_iter()).for_each(|(d2, d1)|{
      //         *d2 ^= *d1;
      //     });
      //     // 压缩
      //     unsafe {
      //         pres_data.set_len(0);
      //     }
      //     let clen = ctx.compress(&mut pres_data, &data2, COMPRESS_LEVEL).unwrap();
      //     // let clen = util::compress(&data2, &mut pres_data);
      //     // 发送diff
      //     encode(clen, &mut header);
      //     if let Err(_) = stream.write_all(&header) {
      //         return;
      //     }
      //     if let Err(_) = stream.write_all(&pres_data[..clen]) {
      //         return;
      //     }
      //     util::skip(clen);
      //     break;
      // }
  }
    
}






// fn compress_diff_image(width: u32, height: u32, pixels: &[u8]) {
//   let mut img = ImageBuffer::new(width, height);

//   for (x, y, pixel) in img.enumerate_pixels_mut() {
//       let offset = (y * width + x) as usize;
//       let r = pixels[offset];
//       let g = pixels[offset + 1];
//       let b = pixels[offset + 2];
//       *pixel = Rgb([r, g, b]);
//   }

//   // 保存压缩的差异化图像
//   img.save("target/storage/diff.png").unwrap();

// }

// 用遍历的方式捕捉当前的正确帧。
	  // // 捕捉固定区域的图片，并写到缓冲区
    // let image2 = screen.capture_area(display.x, display.y, display.width, display.height).unwrap();
    // let buffer2 = image2.buffer();
    
    // // 将缓冲区的数据保存为 png 文件
    // fs::write("target/storage/screen2.png", &buffer2).unwrap();
  
    // // 读入这两个图片
    // let img1 = image::open("target/storage/screen1.png").unwrap();
    // let img2 = image::open("target/storage/screen2.png").unwrap();

    // // 将两个图片转换为RGB图像
    // let img1 = img1.into_rgb16();
    // let img2 = img2.into_rgb16();

    // // 获取图像的宽度和高度
    // let (width, height) = img1.dimensions();

    // // 创建一个新图像，用于存储差异化图像
    // let mut diff_img = ImageBuffer::new(width, height);

    // // 遍历图像的每一个像素
    // for y in 0..height {
    //   for x in 0..width {
    //       // 获取两个图像中当前像素的RGB值
    //       let px1 = img1.get_pixel(x, y);
    //       let px2 = img2.get_pixel(x, y);

    //       // 如果两个像素都是黑色，则不进行处理
    //       if px1[0] == 0 && px1[1] == 0 && px1[2] == 0 && px2[0] == 0 && px2[1] == 0 && px2[2] == 0 {
    //         continue;
    //       }

    //       // 计算两个图像中当前像素的差值
    //       let diff = (px1[0] as i32 - px2[0] as i32).abs()
    //           + (px1[1] as i32 - px2[1] as i32).abs()
    //           + (px1[2] as i32 - px2[2] as i32).abs();
          
    //       // 将差值作为新图像的当前像素的RGB值
    //       diff_img.put_pixel(x, y, image::Rgb([diff as u8, diff as u8, diff as u8]));
    //   }
    // }

    // // pixels 是一个包含像素数据的切片，创建一个长度为 width * height * 3 的切片，用于存储像素数据
    // // 像素数据应该是一个长度为 width * height * 3 的切片，每 3 个元素表示一个像素点的 RGB 颜色值
    // let pixels = vec![0; (diff_img.width() * diff_img.height() * 3).try_into().unwrap()];

    // // 调用压缩函数
    // compress_diff_image(diff_img.width(), diff_img.height(), &pixels);

    // // 保存压缩的差异化图像
    // // diff_img.save("target/storage/diff.png").unwrap();
    // // diff_img.save("target/storage/diff.png").unwrap();
