fn idx2(x: usize, y: usize, pitch: usize) -> usize {
    y * pitch + x
}

struct CubeDrawer {
    input: String,
    pitch: usize,
    len: usize,
    canvas: Vec<char>,
}

impl CubeDrawer {
    fn draw(input: &str) -> String {
        let len = input.chars().count();
        let mut canvas = Vec::new();

        let width = len * 3;
        let height = len * 3 / 2;
        let pitch = width + 1;
        for _ in 0..height {
            for _ in 0..width {
                canvas.push(' ');
            }
            canvas.push('\n');
        }
        canvas.pop();

        let mut cube_drawer = CubeDrawer {
            input: input.to_uppercase(),
            pitch,
            len,
            canvas,
        };

        cube_drawer.draw2d(if len % 2 == 0 { len } else { len - 1 }, 0);
        cube_drawer.draw2d(0, len / 2);
        cube_drawer.draw_diagonals();

        let mut output = String::new();
        for c in cube_drawer.canvas {
            output.push(c);
        }
        output
    }

    fn draw2d(&mut self, start_x: usize, start_y: usize) {
        for (i, c) in self.input.chars().enumerate() {
            // Top row
            self.canvas[idx2(i * 2 + start_x, start_y, self.pitch)] = c;
            // Left column
            self.canvas[idx2(start_x, i + start_y, self.pitch)] = c;
            // Bottom row
            self.canvas[idx2(
                (self.len - 1) * 2 - (i * 2) + start_x,
                start_y + self.len - 1,
                self.pitch,
            )] = c;
            // Right column
            self.canvas[idx2(
                (self.len - 1) * 2 + start_x,
                start_y + self.len - 1 - i,
                self.pitch,
            )] = c;
        }
    }

    fn draw_diagonals(&mut self) {
        for i in 1..self.len / 2 {
            self.canvas[idx2((self.len / 2 - i) * 2, i, self.pitch)] = '/';
            self.canvas[idx2((self.len / 2 - i) * 2, i + self.len - 1, self.pitch)] = '/';
            self.canvas[idx2((self.len / 2 - i) * 2 + self.len * 2 - 2, i, self.pitch)] = '/';
            self.canvas[idx2(
                (self.len / 2 - i) * 2 + self.len * 2 - 2,
                i + self.len - 1,
                self.pitch,
            )] = '/';
        }
    }
}

pub fn draw(input: &str) -> String {
    CubeDrawer::draw(input)
}
