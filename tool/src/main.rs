//(-1.0, -1.0) (-0.9, -1.0)  0 2 4 ...
//(-1.0, -0.9) (-0.9, -0.9)  1 3 5

// 0 1 2 3  0 1 21 /  0 20 21
// 21 22
//
// 0 1 22
// 0 21 22
// ..
// 19 40 41
// 19 20 41
// ..
// 21 22 43
// 21 42 43
//
// 21 * i (i in 0..20)
// 
// 
// 

//pub const VERTICES: [Vertex; 441] = [
//    Vertex { position: (-1.0, -1.0) }, 

fn main() {
    //VERTICES
    println!("pub const VERTICES: [Vertex; 441] = [");
    for y in 0..21 {
        let reg_y = -1.0 * (y - 10) as f32 / 10.0;
        for x in 0..21 {
            let reg_x = (x - 10) as f32 / 10.0;
            println!("Vertex {{ position: ({:.1}, {:.1})}},", reg_x, reg_y);
        }
    }
    println!("];");

    //INDECIES
    println!("pub const INDICES: [u16; 2400] = [");
    for y in 0..20 {
        for x in 0..20 {
            // c0 c1
            // c2 c3
            let c0 = x + (y * 21);
            let c1 = x + (y * 21) + 1; 
            let c2 = x + (y * 21) + 21;  
            let c3 = x + (y * 21) + 22;  

            println!("{}, {}, {},", c0 + 1, c1 + 1, c3 + 1);
            println!("{}, {}, {},", c0 + 1, c2 + 1, c3 + 1);
        }
    }
    println!("];");

}
