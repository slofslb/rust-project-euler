fn main() {
    let data = std::fs::read_to_string("p102_triangles.txt").expect("打开文件出错");
    let data2 = data.replace("\r\n", "\n");
    let lines = data2.trim().split('\n');
    let mut count = 0;
    for line in lines {
        let pts: Vec<_> = line.split(',')
                              .map(|x| x.parse::<f64>().unwrap())
                              .collect();
        if origin_in_triangle(pts[0], pts[1],pts[2], pts[3],pts[4], pts[5]) {
            println!("{:?}", pts);
            count += 1;
        }
    }
    println!("{}", count );
}
// 228

fn origin_in_triangle(x1:f64, y1:f64, x2:f64, y2:f64, x3:f64, y3:f64) -> bool {
    let t1 = x2 * y1 - x1 * y2;
    let t2 = x3 * y2 - x2 * y3;
    let t3 = x1 * y3 - x3 * y1;
    (t1 > 0.0 && t2 > 0.0 && t3 > 0.0) || (t1 < 0.0 && t2 < 0.0 && t3 < 0.0)
}