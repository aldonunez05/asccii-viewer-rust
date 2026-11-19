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

fn triangularize_recurse(vecs: &[Vec3], idx: &mut Vec<usize>, orient: bool, out_idxs: &mut Vec<usize>){
	let n = idxs.len();
	assert!(n >= 3);

	if n == 3{
		out_idx[..3].copy_from_slice(&idxs[..3]);
		return;
	}
	
 	//find the convex angle
	let (mut i1, mut i2, mut i3) = (0,0,0);
	for t in 0..n{
		i1 = (n / 2 + t + (n - 1)) % n;
		i2 = (n / 2 + t) % n;
		i3 = (n / 2 + t + 1) % n;

		let d1 = v3 - v2;
		let d2 = v1 - v2;
		let cross_prod = d1.cross(d2).z;
		let convex = cross_prod == 0 || ((cross_prod > 0.0 != orient));

		if convex{
			break;
		}
	}
	let (v1, v2, v3) = (vecs[i1], vecs[i2], vecs[i3]);
	
	//rect equation ax + by + c = 0 for the line between v1 and v3
	let a = v1.y - v3.y;
	let b = v3.x - v1.x;
	let c = (v1.x - v3.x) * v1.y + (v3.y - v1.y) * v1.x;
	//find point inside the (v1, v2, v3) triangle with the largest perpendicular distanceto line (v1,v3)
	let mut max_dist_k: Option<usize> = None; 
	let mut max_dist = 0.0f32;
	
	for k in 0..n{
		if k == i1 || k == i2 || k == i3{
			continue;
		}
		if point_in_triangle(vecs[k], v1, v2, v3)){
			//perpendicular distance (multiplied by sqrt(a^2 + b^2))
			let dist =(a * vecs[k].x + b * vecs[k].y + c).abs();

			if max_dist_k.is_none() || dist > max_dist{
				max_dist = dist;
				max_dist_k = Some(k);
			}
		}
	}
	match max_dist_k{
		None => {
			out_idxs[0] = idxs[i1];
			out_idxs[1] = idxs[i2];
			out_idxs[2] = idxs[i3];

			vecs.remove(i2);
			idxs.remove(i2);
			triangularize_recurse(vecs, idxs, orient, &mut out_idxs[3..]);
		}
		Some(k) =>{
			//create a diagonal from i2 to k, split the problem in 2
			let mut vecs1 = Vec::with_capacity(n);
			let mut vecs2 = Vec::with_capacity(n);
			let mut idxs1 = Vec::with_capacity(n):
			let mut idxs2 = Vec::with_capacity(n):

			let mut side = false;
			for r in 0..n{
				if r == i2 || r == k{
					vecs1.push(vecs[r]);
					idxs1.push(idxs[r]);
					vecs2.push(vecs[r]);
					idxs2.push(idxs[r]);
					side = !side;
				} else if side{
					vecs1.push(vecs[r]);
					idxs1.push(idxs[r]);
				} else {
					vecs2.push(vecs[r]);
					idxs2.push(idxs[r]);
				}
			}
			let n1 = idxs1.len();
			assert!(n1 + idxs2.len() == n + 2);
			triangularize_recurse(vecs1, idxs1, orient, out_idxs);
			triangularize_recurse(vecs2, idxs2, orient, &mut out_idxs[3 * (n1 - 2)..]);
		}
	}
} 

fn triangularize(const vecs: &[Vecs3]) -> Vec<usize>){
	let n = vecs.len();
	assert!(n >= 3);

	//find the plane that contains all vectors (given by <dir1, dir2>)
	let mut best_normal_mag = 0.0f32;
	let mut dir1 = Vec3::ZERO;
	let mut dir2 = VEC3::ZERO;

	for i in 0..n{
		let v1 = vecs[i];
		let v2 = vecs[(i + 1) % n];
		let v3 = vecs[(i + 2) % n];

		let d1 = v1 - v2;
		let d2 = v3 - v2;
		let normal = d1.cross(d2);
		let mag = normal.length();

		if best_normal_mag <= mag{
			best_normal_mag = mag;
			dir1 = d1.normalize();
			dir2 = normal.normalize().cross(d1);
		}
	}
	let mut area = 0.0f32;
	for i in 0..n{
		let v1 = vecs_plane[i];
		let v2 = vecs_plane[(i + 1) % n];
		area += (v2.x - v1.x) * (v2.y + v1.y);
	}
	let orientation = area >= 0.0;

	let idxs: Vec<usize> = (0..n).collect();

	let mut out_idxs = vec![0usize; 3 * (n - 2)];
	triangularize_recurse(vecs_plane, idxs, orientation, &mut out_idxs);
	out_idxs
} 
