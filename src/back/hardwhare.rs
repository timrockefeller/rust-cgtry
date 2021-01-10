#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;
#[cfg(not(any(feature = "vulkan", feature = "metal", feature = "dx12",)))]
use gfx_backend_vulkan as back;

// pub struct HardwareState {
//     current_frame: usize,    // 当前帧
//     frames_in_flight: usize, // 当前传输的帧
//     in_flight_fences: Vec<<back::Backend as Backend>::Fence>,                         // 正在运行中的内存屏障
//     render_finished_semaphores: Vec<<back::Backend as Backend>::Semaphore>,           // 渲染结束时发送的信号
//     image_available_semaphores: Vec<<back::Backend as Backend>::Semaphore>,           // 可用信号
//     command_buffers: Vec<CommandBuffer<back::Backend, Graphics, MultiShot, Primary>>, // 命令缓冲区域
//     command_pool: ManuallyDrop<CommandPool<back::Backend, Graphics>>,                 // 命令池
//     framebuffers: Vec<<back::Backend as Backend>::Framebuffer>,                       // 帧缓冲
//     image_views: Vec<(<back::Backend as Backend>::ImageView)>,         // 图像视图
//     render_pass: ManuallyDrop<<back::Backend as Backend>::RenderPass>, // 渲染过程
//     render_area: Rect,                                                 // 渲染区域
//     queue_group: QueueGroup<back::Backend, Graphics>,                  // 队列簇
//     swapchain: ManuallyDrop<<back::Backend as Backend>::Swapchain>,    // 交换链
//     device: ManuallyDrop<back::Device>,                                // 设备
//     _adapter: Adapter<back::Backend>,              //用来抽象设备的转接器
//     _surface: <back::Backend as Backend>::Surface, //你的屏幕
//     _instance: ManuallyDrop<back::Instance>,       //示例
// }
