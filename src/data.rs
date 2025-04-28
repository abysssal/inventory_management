pub mod data {
    struct CPU {
        name: String,
        price: f32,
        quantity: i8
    }
    
    struct GPU {
        name: String,
        price: f32,
        vram: i8,
        quantity: i8
    }
    
    struct Motherboard {
        name: String,
        price: i32,
        socket: String,
        chipset: String,
        quantity: i8
    }
}