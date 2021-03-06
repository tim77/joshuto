use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::{FileType, JoshutoDirList};
use crate::util::format;

const FILE_SIZE_WIDTH: usize = 8;

pub struct TuiDirListDetailed<'a> {
    dirlist: &'a JoshutoDirList,
}

impl<'a> TuiDirListDetailed<'a> {
    pub fn new(dirlist: &'a JoshutoDirList) -> Self {
        Self { dirlist }
    }
}

impl<'a> Widget for TuiDirListDetailed<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 1 {
            return;
        }

        let x = area.left();
        let y = area.top();
        let curr_index = match self.dirlist.index {
            Some(i) => i,
            None => {
                let style = Style::default().bg(Color::Red).fg(Color::White);
                buf.set_stringn(x, y, "empty", area.width as usize, style);
                return;
            }
        };

        let skip_dist = curr_index / area.height as usize * area.height as usize;
        let screen_index = curr_index % area.height as usize;

        let area_width = area.width as usize;
        for (i, entry) in self
            .dirlist
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
        {
            let name = entry.label();
            let name_width = name.width();

            let style = if i == screen_index {
                entry.get_style().add_modifier(Modifier::REVERSED)
            } else {
                entry.get_style()
            };

            let file_type = &entry.metadata.file_type;
            match file_type {
                FileType::Directory => {
                    if name_width <= area_width {
                        buf.set_string(x, y + i as u16, name, style);
                    } else {
                        buf.set_stringn(x, y + i as u16, name, area_width - 1, style);
                        buf.set_string(x + area_width as u16 - 1, y + i as u16, "…", style);
                    }
                }
                FileType::Symlink(p) => {
                    if name_width < area_width - 4 {
                        buf.set_string(x, y + i as u16, name, style);
                        buf.set_string(x + area_width as u16 - 4, y + i as u16, "->", style);
                    } else {
                        buf.set_stringn(x, y + i as u16, name, area_width - 1, style);
                        buf.set_string(x + area_width as u16 - 1, y + i as u16, "…", style);
                    }
                }
                FileType::File => {
                    if name_width < area_width - FILE_SIZE_WIDTH {
                        buf.set_stringn(x, y + i as u16, name, area_width - FILE_SIZE_WIDTH, style);
                    } else {
                        match name.rfind('.') {
                            None => {
                                buf.set_stringn(
                                    x,
                                    y + i as u16,
                                    name,
                                    area_width - FILE_SIZE_WIDTH,
                                    style,
                                );
                            }
                            Some(p_ind) => {
                                let ext_width = name[p_ind..].width();
                                let file_name_width =
                                    if ext_width > area_width - FILE_SIZE_WIDTH - 2 {
                                        0
                                    } else {
                                        area_width - FILE_SIZE_WIDTH - ext_width - 2
                                    };

                                buf.set_stringn(
                                    x,
                                    y + i as u16,
                                    &name[..p_ind],
                                    file_name_width,
                                    style,
                                );
                                buf.set_string(
                                    x + file_name_width as u16,
                                    y + i as u16,
                                    "…",
                                    style,
                                );

                                let file_ext_width =
                                    area_width - file_name_width - FILE_SIZE_WIDTH - 2;

                                buf.set_stringn(
                                    x + file_name_width as u16 + 1,
                                    y + i as u16,
                                    &name[p_ind..],
                                    file_ext_width,
                                    style,
                                );
                            }
                        }
                    }
                    let file_size_string = format::file_size_to_string(entry.metadata.len);
                    buf.set_string(
                        x + (area_width - FILE_SIZE_WIDTH) as u16,
                        y + i as u16,
                        file_size_string,
                        style,
                    );
                }
            }
        }
    }
}
