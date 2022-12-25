use rayon::prelude::*;
use scrap::{Capturer, Display};
use std::{
    thread,
    time::Duration,
    io::ErrorKind::WouldBlock,
};

pub struct ScreenCaptureStruct {
    display: DisplayStruct,
    capturer: Option<Capturer>,
    sleep: Duration,
}

struct DisplayStruct {
    width: usize,
    height: usize,
    size: usize, // 后面控制端可以通过此参数自行选择获取屏幕
}

impl ScreenCaptureStruct {
    pub fn instance() -> Self {
        // 0: Display::primary() 方法用于选择要捕捉主显示器，
        //    该方法返回一个 scrap::Display 类型的结构体，表示主显示器的信息。
        // 1: Display::all() 方法获取所有显示器的列表，
        //    该方法返回一个 Vec<scrap::Display> 的动态数组类型，表示所有显示器的信息。
        //    需要注意：显示器集合是以屏幕左上角为第一个元素，以屏幕右下角为最后一个元素排序的。
        let displays = match Display::all() {
            Ok(displays) => {
            // 获取全部可用屏幕，并将其保存后，检查其长度.
            if displays.len() == 0 {
                panic!("无法找到可以用的屏幕");
            }
            displays
            // 移除并返回向量中位置索引处的元素，将其后的所有元素向左移动。
            // displays.into_iter().next().unwrap()
            // displays.remove(0)
            },
            Err(_) => panic!("无法找到屏幕信息"),
        };

        // 将捕捉结构体中的宽度、高度和大小保存到二元组中。
        let (width, height, size) = (displays[0].width(), displays[0].height(), displays.len());
    
        // 0: Capturer::new(display) 方法用于创建一个用于捕捉屏幕图片的对象，
        //    该方法接受一个 Display 类型的参数，表示要捕捉的显示器。
        let capturer = match Capturer::new(displays.into_iter().next().unwrap()) {
            Ok(display) => display,
            Err(_) => panic!("无法捕捉屏幕"),
        };

        Self {
            display: DisplayStruct {
                width,
                height,
                size,
            },
            // 如果捕获成功，那么它会返回一个 ScreenCapturer 类型的变量，否则会返回一个 None
            capturer: Some(capturer),
            // 该函数接受两个参数，分别是以秒为单位的时间和以纳秒为单位的时间。
            // 将 1 秒钟和 0 纳秒（即 1 纳秒）除以 60，得到的结果是 1 纳秒除以 60，即 0.01666666666666666 纳秒
            sleep: Duration::new(1, 0) / 60,
        }
    }

    pub fn start(&mut self, src_frame: &mut [u8]) {
        // 用遍历的方式捕捉当前的正确帧。
        loop {
            match &mut self.capturer {
                // 捕获成功
                Some(capturer) => {
                    // 0: capturer.frame() 方法用于捕捉屏幕图片，它会返回一个 Frame 结构体，
                    //    可以在接下来的代码中执行图像处理算法，处理视频流中的帧，或执行任何其他操作，以便使用当前帧的数据。
                    let frame = match capturer.frame() {
                        Ok(frame) => frame,
                        // 如果尚未准备好，则返回 WouldBlock。
                        Err(error) => {
                            // 暂停线程 1 s
                            thread::sleep(self.sleep);
                            // 判断错误类型
                            if error.kind() == WouldBlock {
                              // println!("Keep spinning.{}", error);
                              continue;
                            } else {
                              // println!("reload.");
                              thread::sleep(Duration::from_millis(200));
                              self.reload();
                              continue;
                            }
                        }
                    };
                    
                    // 先使用 par_chunks_exact_mut 方法将 pixels_buffer 分块成大小为 3 的块，以便进行并行操作，
                    let chunks = src_frame.par_chunks_exact_mut(3);

                    // 然后使用 zip 方法将 pixels_buffer 和 frame 两个并行切片配对，
                    let new_chunks = chunks.zip(frame.par_chunks_exact(4));

                    // 最后使用 for_each 方法遍历每个元组，将原来的每个像素和 0b1111_1000 进行二进制的按为位与运算，右边0越多压缩程度越大
                    // 0b1111_1000 的二进制位表示的是一个掩码，它可以用于提取某个二进制数中的特定位。
                    // 如果将它与某个二进制数进行按位与运算，则会提取出该二进制数中最低有效位以及前三位，
                    // 例如：1010 & 1111_1000，其结果为 1000，这就是按位与运算的结果。
                    new_chunks.for_each(|(new, old)| {
                        new[0] = old[0] & 0b1111_1000;
                        new[1] = old[1] & 0b1111_1000;
                        new[2] = old[2] & 0b1111_1000;
                    });
                    break;
                },
                // 捕获失败
                None => {
                    std::thread::sleep(Duration::from_millis(200));
                    self.reload();

                    continue;
                },
            }
        }
    }

    pub fn reload(&mut self) {
        print!("屏幕捕捉-重新捕捉帧...\n");

        // 释放上之前的捕捉
        drop(self.capturer.take());

        // 再次获取全部屏幕
        let displays = match Display::all() {
            Ok(displays) => {
            if displays.len() == 0 {
                panic!("无法找到可以用的屏幕");
            }
            displays
            },
            Err(_) => panic!("无法找到屏幕信息"),
        };
        let capturer = match Capturer::new(displays.into_iter().next().unwrap()) {
            Ok(display) => display,
            Err(_) => panic!("无法捕捉屏幕"),
        };
        self.capturer = Some(capturer);
    }

    pub fn display(&self) -> (usize, usize, usize) {
        return (self.display.width, self.display.height, self.display.size);
    }
}