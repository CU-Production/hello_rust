#![allow(dead_code)] 
use cgmath::*; 
mod math_func; 

pub fn cube_data() -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<[i8; 2]>, Vec<[i8; 3]>) { 
    let positions = [ 
        // front (0, 0, 1)
        [-1, -1, 1], [1, -1, 1], [-1, 1, 1], [-1, 1, 1], [ 1, -1, 1], [ 1, 1, 1], 
        // right (1, 0, 0)
        [ 1, -1, 1], [1, -1, -1], [ 1, 1, 1], [ 1, 1, 1], [ 1, -1, -1], [ 1, 1, -1], 
        // back (0, 0, -1)
        [ 1, -1, -1], [-1, -1, -1], [1, 1, -1], [ 1, 1, -1], [-1, -1, -1], [-1, 1, -1], 
        // left (-1, 0, 0)
        [-1, -1, -1], [-1, -1, 1], [-1, 1, -1], [-1, 1, -1], [-1, -1, 1], [-1, 1, 1], 
        // top (0, 1, 0)
        [-1, 1, 1], [ 1, 1, 1], [-1, 1, -1], [-1, 1, -1], [ 1, 1, 1], [ 1, 1, -1],
        // bottom (0, -1, 0)
        [-1, -1, -1], [ 1, -1, -1], [-1, -1, 1], [-1, -1, 1], [ 1, -1, -1], [ 1, -1, 1], 
    ]; 

    let colors = [ 
        // front - blue
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], 
        // right - red
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], 
        // back - yellow 
        [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], 
        // left - aqua
        [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], 
        // top - green
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], 
        // bottom - fuchsia
        [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], 
    ]; 

    let uvs= [ 
        // front
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1], 
        // right
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1], 
        // back
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1], 
        // left
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1], 
        // top
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1], 
        // bottom
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1], 
    ]; 

    let normals = [ 
        // front 
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], 
        // right 
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], 
        // back 
        [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], 
        // left 
        [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], 
        // top 
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], 
        // bottom
        [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], 
    ]; 

    // return data
    (positions.to_vec(), colors.to_vec(), uvs.to_vec(), normals.to_vec()) 
} 

pub fn cube_data_index() -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<u16>) { 
    let positions = [ 
        [-1, -1,  1], // vertex a
        [ 1, -1,  1], // vertex b
        [ 1,  1,  1], // vertex c
        [-1,  1,  1], // vertex d
        [-1, -1, -1], // vertex e
        [ 1, -1, -1], // vertex f
        [ 1,  1, -1], // vertex g
        [-1,  1, -1], // vertex h
    ]; 

    let colors = [ 
        [0, 0, 1], // vertex a
        [1, 0, 1], // vertex b
        [1, 1, 1], // vertex c
        [0, 1, 1], // vertex d
        [0, 0, 0], // vertex e
        [1, 0, 0], // vertex f
        [1, 1, 0], // vertex g
        [0, 1, 0], // vertex h
    ]; 

    let indices = [ 
        0, 1, 2, 2, 3, 0, // front 
        1, 5, 6, 6, 2, 1, // right 
        4, 7, 6, 6, 5, 4, // back 
        0, 3, 7, 7, 4, 0, // left 
        3, 2, 6, 6, 7, 3, // top 
        0, 4, 5, 5, 1, 0, // bottom
    ]; 

    (positions.to_vec(), colors.to_vec(), indices.to_vec()) 
} 

pub fn sphere_data(r: f32, u:usize, v:usize) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) { 
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((4* (u - 1)*(v -1)) as usize); 
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((4* (u - 1)*(v -1)) as usize); 
    let uvs: Vec<[f32; 3]> = Vec::with_capacity((4* (u - 1)*(v -1)) as usize); 
    for i in 0..u - 1 { 
        for j in 0..v - 1 { 
            let theta = i as f32 *180.0/(u as f32 - 1.0); 
            let phi = j as f32 * 360.0/(v as f32 - 1.0); 
            let theta1 = (i as f32 + 1.0) *180.0/(u as f32 - 1.0); 
            let phi1 = (j as f32 + 1.0) * 360.0/(v as f32 - 1.0); 
            let p0 = math_func::sphere_position(r, Deg(theta) , Deg(phi)); 
            let p1 = math_func::sphere_position(r, Deg(theta1) , Deg(phi)); 
            let p2 = math_func::sphere_position(r, Deg(theta1) , Deg(phi1)); 
            let p3 = math_func::sphere_position(r, Deg(theta) , Deg(phi1)); 

            // positions
            positions.push(p0); 
            positions.push(p1); 
            positions.push(p3); 
            positions.push(p1); 
            positions.push(p2); 
            positions.push(p3); 

            // normals
            normals.push([p0[0]/r, p0[1]/r, p0[2]/r]); 
            normals.push([p1[0]/r, p1[1]/r, p1[2]/r]); 
            normals.push([p3[0]/r, p3[1]/r, p3[2]/r]); 
            normals.push([p1[0]/r, p1[1]/r, p1[2]/r]); 
            normals.push([p2[0]/r, p2[1]/r, p2[2]/r]); 
            normals.push([p3[0]/r, p3[1]/r, p3[2]/r]); 
        } 
    } 
    (positions, normals, uvs) 
} 

pub fn cone_data(rtop: f32, rbottom: f32, height: f32, n:usize) -> 
    (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) { 
    let h = height / 2.0; 
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(12 * (n-1)); 
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(12 * (n -1)); 
    let uvs: Vec<[f32; 3]> = Vec::with_capacity(12 * (n -1)); 
    for i in 0..n - 1 { 
        let theta = i as f32 *360.0/(n as f32 - 1.0); 
        let theta1 = (i as f32 + 1.0) *360.0/(n as f32 - 1.0); 
        let p0 = math_func::cylinder_position(rtop, h, Deg(theta)); 
        let p1 = math_func::cylinder_position(rbottom, -h, Deg(theta)); 
        let p2 = math_func::cylinder_position(0.0, -h, Deg(theta)); 
        let p3 = math_func::cylinder_position(0.0, h, Deg(theta)); 
        let p4 = math_func::cylinder_position(rtop, h, Deg(theta1)); 
        let p5 = math_func::cylinder_position(rbottom, -h, Deg(theta1)); 

        // positions
        // top face
        positions.push(p0); 
        positions.push(p4); 
        positions.push(p3); 

        // bottom face
        positions.push(p1); 
        positions.push(p2); 
        positions.push(p5); 

        // outer face
        positions.push(p0); 
        positions.push(p1); 
        positions.push(p5); 
        positions.push(p5); 
        positions.push(p4); 
        positions.push(p0); 

        // normals
        let ca = Vector3::new(p5[0]-p0[0], p5[1]-p0[1], p5[2]-p0[2]); 
        let db = Vector3::new(p4[0]-p1[0], p4[1]-p1[1], p4[2]-p1[2]); 
        let cp = (ca.cross(db)).normalize(); 

        // top face
        normals.push([0.0, 1.0, 0.0]); 
        normals.push([0.0, 1.0, 0.0]); 
        normals.push([0.0, 1.0, 0.0]); 

        // bottom face
        normals.push([0.0, -1.0, 0.0]); 
        normals.push([0.0, -1.0, 0.0]); 
        normals.push([0.0, -1.0, 0.0]); 

        // outer face
        normals.push([cp[0], cp[1], cp[2]]); 
        normals.push([cp[0], cp[1], cp[2]]); 
        normals.push([cp[0], cp[1], cp[2]]); 
        normals.push([cp[0], cp[1], cp[2]]); 
        normals.push([cp[0], cp[1], cp[2]]); 
        normals.push([cp[0], cp[1], cp[2]]); 
    } 
    (positions, normals, uvs) 
} 

pub fn torus_data(r_torus:f32, r_tube:f32, n_torus:usize, n_tube:usize) -> 
    (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) { 
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((4* (n_torus - 1)*(n_tube -1)) as usize); 
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((4* (n_torus - 1)*(n_tube -1)) as usize); 
    let uvs: Vec<[f32; 3]> = Vec::with_capacity((4* (n_torus - 1)*(n_tube -1)) as usize); 
    for i in 0..n_torus - 1 { 
        for j in 0..n_tube - 1 { 
            let u = i as f32 * 360.0/(n_torus as f32 - 1.0); 
            let v = j as f32 * 360.0/(n_tube as f32 - 1.0); 
            let u1 = (i as f32 + 1.0) * 360.0/(n_torus as f32 - 1.0); 
            let v1 = (j as f32 + 1.0) * 360.0/(n_tube as f32 - 1.0); 
            let p0 = math_func::torus_position(r_torus, r_tube, Deg(u), Deg(v)); 
            let p1 = math_func::torus_position(r_torus, r_tube, Deg(u1), Deg(v)); 
            let p2 = math_func::torus_position(r_torus, r_tube, Deg(u1), Deg(v1)); 
            let p3 = math_func::torus_position(r_torus, r_tube, Deg(u), Deg(v1)); 

            // positions
            positions.push(p0); 
            positions.push(p1); 
            positions.push(p2); 
            positions.push(p2); 
            positions.push(p3); 
            positions.push(p0); 

            // normals
            let ca = Vector3::new(p2[0]-p0[0], p2[1]-p0[1], p2[2]-p0[2]); 
            let db = Vector3::new(p3[0]-p1[0], p3[1]-p1[1], p3[2]-p1[2]); 
            let cp = (ca.cross(db)).normalize(); 
            normals.push([cp[0], cp[1], cp[2]]); 

            normals.push([cp[0], cp[1], cp[2]]); 
            normals.push([cp[0], cp[1], cp[2]]); 
            normals.push([cp[0], cp[1], cp[2]]); 
            normals.push([cp[0], cp[1], cp[2]]); 
            normals.push([cp[0], cp[1], cp[2]]); 
        } 
    } 
    (positions, normals, uvs) 
} 
