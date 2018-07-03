pub struct Grid {
    grid: Vec<bool>,
}

impl Grid {
    fn is_alive_at(x: usize, y: usize) {}
}

#[cfg(test)]
mod test {
    #[test]
    fn grid_has_correct_width() {
        let grid = Grid::new(10, 5);
        assert_eq!(10, grid.width());
    }

    #[test]
    fn grid_has_correct_height() {
        let grid = Grid::new(10, 5);
        assert_eq!(5, grid.height());
    }

    #[test]
    fn grid_inits_dead() {
        let grid = Grid::new(10, 10);
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                assert_eq!(false, grid.is_alive_at(x, y))
            }
        }
    }
}
