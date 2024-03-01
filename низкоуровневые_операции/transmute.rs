fn main() {
    let l = Line { start: Point { x: 1, y: 2 }, end: Point { x: 3, y: 4 } };
    let transmuted_start = unsafe { std::mem::transmute::
    <Point, Point>(l.start) };
    let transmuted_end = unsafe { std::mem::transmute::
    <Point, Point>(l.end) };
    let transmuted_line = Line { start: transmuted_start, end: transmuted_end };
    println!("{}", transmuted_line.start.x);

}
#[repr(C)]//lang = "C",
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[repr(C)]
struct Line {
    start: Point,
    end: Point,
}