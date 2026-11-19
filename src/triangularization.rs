use glam::Vec3;

fn absfloat(a : f32) -> f32{
	a.abs()
}

fn triangle_area(pt: Vec3, p2: Vec3, p3: Vec3) -> f32{
	(p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y)).abs() / 2.0
}

fn point_in_triangle(pt: Vec3, v1: Vec3, v2: Vec3, v3: Vec3)-> bool{
	let atot = triangle_area(v1, v2, v3);
	let a1 = triangle_area(v1, v2, pt);
	let a2 = triangle_area(v2, v3, pt);
	let a3 = triangle_area(v3, v1, pt);

	a1 + a2 + a3 <= atot * 1.00001
}

fn triangularize_recursive(vecs: &[Vec3], idx: &mut Vec<usize>, orient: bool, out_idxs: &mut Vec<usize>){

} 
