pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        let (x, y) = size;
        Ok(Self {
            size: Size {
                width: x,
                height: y,
            },
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
}
