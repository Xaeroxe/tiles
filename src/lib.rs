#[derive(Debug, Clone)]
pub struct TileMap<T> {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub tiles: Vec<T>,
}

pub trait Tile {
    fn collides(&self) -> bool;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct Mesh {
    vertices: Vec<Point>,
    index_buffer: Vec<usize>,
}

impl Mesh {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            index_buffer: Vec::new(),
        }
    }

    fn add_cube(&mut self, cube_origin: Point) {
        let offsets = [[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1], [1, 1, 0], [0, 1, 1], [1, 0, 1], [1, 1, 1]];

    }
}

pub struct ColliderIterator<'a, T> {
    map: &'a TileMap<T>,
    visited: Vec<Point>,
    cursor: Point,
}

impl<'a, T> ColliderIterator<'a, T> {
    fn new(map: &'a TileMap<T>) -> Self {
        Self {
            map,
            visited: Vec::new(),
            cursor: Point {x: 0, y: 0, z: 0},
        }
    }

    fn advance_cursor(&mut self) -> bool {
        if self.cursor.x == self.map.width && self.cursor.y == self.map.height && self.cursor.z == self.map.depth {
            return false;
        }
        self.visited.push(self.cursor);
        self.cursor.x += 1;
        if self.cursor.x == self.map.width {
            self.cursor.x = 0;
            self.cursor.y += 1;
            if self.cursor.y == self.map.height {
                self.cursor.y = 0;
                self.cursor.z += 1;
                if self.cursor.z == self.map.depth {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<'a, T: Tile> Iterator for ColliderIterator<'a, T> {
    type Item = Mesh;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.map.tile_at(&self.cursor).collides() && !self.visited.contains(&self.cursor) {
            if !self.advance_cursor() {
                return None;
            }
        }
        self.visited.push(self.cursor);
        let mut mesh = Mesh::new();
        let mut neighbors_not_visited = Vec::new();
        mesh.add_cube(self.cursor);
        neighbors_not_visited.push(self.cursor);
        while let Some(new_tile) = neighbors_not_visited.pop() {
            let offsets = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];
            for offset in offsets.iter() {
                let neighbor = Point {
                    x: new_tile.x + offset[0],
                    y: new_tile.y + offset[1],
                    z: new_tile.z + offset[2],
                };
                if self.map.tile_at(&neighbor).collides() && !self.visited.contains(&neighbor) {
                    neighbors_not_visited.push(neighbor);
                    self.visited.push(neighbor);
                    mesh.add_cube(neighbor);
                }
            }
        }

        None
    }
}

impl<T> TileMap<T>
{
    /// Tile origin is at the top-front-left vertex of the cube.
    pub fn tile_at(&self, point: &Point) -> &T {
        &self.tiles[(point.z * self.width * self.height + point.y * self.width + point.x) as usize]
    }

    pub fn colliders(&self) -> ColliderIterator<T>
    where T: Tile
    {
        ColliderIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
