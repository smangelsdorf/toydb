const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;

pub struct Table {
    num_rows: u32,
    pages: [Option<Box<Page>>; TABLE_MAX_PAGES],
}

pub struct Page {
    bytes: [u8; PAGE_SIZE],
}

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;

#[repr(C)]
pub struct Row {
    pub id: u32,
    pub username: [u8; COLUMN_USERNAME_SIZE],
    pub email: [u8; COLUMN_EMAIL_SIZE],
}

// Includes padding for alignment of the next value 🎉
const ROW_SIZE: usize = std::mem::size_of::<Row>();
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = TABLE_MAX_PAGES * ROWS_PER_PAGE;

// HACK: This is needed to initialize the `pages` array without using unsafe Rust.
// https://github.com/rust-lang/rust/issues/49147
// https://github.com/rust-lang/rust/pull/79270
const NO_PAGE: Option<Box<Page>> = None;

impl Table {
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: [NO_PAGE; TABLE_MAX_PAGES],
        }
    }

    pub fn page(&mut self, n: usize) -> Option<&Page> {
        match self.pages.get(n) {
            Some(Some(ref page)) => Some(page),
            _ => None,
        }
    }

    pub fn page_mut(&mut self, n: usize) -> Option<&mut Page> {
        self.pages.get_mut(n).map(|o| {
            o.get_or_insert_with(|| {
                Box::new(Page {
                    bytes: [0; PAGE_SIZE],
                })
            })
            .as_mut()
        })
    }

    pub fn insert(&mut self, row: (u32, &str, &str)) {
        let (id, username_str, email_str) = row;

        let username_bytes = username_str.as_bytes();
        let email_bytes = email_str.as_bytes();

        assert!((self.num_rows as usize) < TABLE_MAX_ROWS);
        assert!(username_bytes.len() < COLUMN_USERNAME_SIZE);
        assert!(email_bytes.len() < COLUMN_EMAIL_SIZE);

        let row_num = self.num_rows;

        self.num_rows += 1;
        let row = self
            .page_mut((row_num as usize) / ROWS_PER_PAGE)
            .and_then(|o| o.row_mut((row_num as usize) % ROWS_PER_PAGE))
            .unwrap();
        row.id = id;
        row.username[0..username_bytes.len()].copy_from_slice(username_bytes);
        row.email[0..email_bytes.len()].copy_from_slice(email_bytes);
    }
}

impl Page {
    pub fn row(&self, n: usize) -> Option<&Row> {
        let start = n * ROW_SIZE;

        // Make sure the last byte is accessible
        let end = start + ROW_SIZE - 1;

        let o = self.bytes.get(end).map(|_| ());
        o.map(|_| unsafe { std::mem::transmute::<*const u8, &Row>(self.bytes.as_ptr().add(start)) })
    }

    pub fn row_mut(&mut self, n: usize) -> Option<&mut Row> {
        let start = n * ROW_SIZE;

        // Make sure the last byte is accessible
        let end = start + ROW_SIZE - 1;

        let o = self.bytes.get(end).map(|_| ());
        o.map(|_| unsafe {
            std::mem::transmute::<*mut u8, &mut Row>(self.bytes.as_mut_ptr().add(start))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_storage() {
        let mut table = Table::new();
        table.insert((1, "test01", "test@example.com"));

        assert_eq!(table.num_rows, 1);
        assert!(table.pages[0].is_some());
        assert!(table.pages[1].is_none());

        table.insert((2, "test02", "test@example.com"));
        table.insert((3, "test03", "test@example.com"));
        table.insert((4, "test04", "test@example.com"));
        table.insert((5, "test05", "test@example.com"));
        table.insert((6, "test06", "test@example.com"));
        table.insert((7, "test07", "test@example.com"));
        table.insert((8, "test08", "test@example.com"));
        table.insert((9, "test09", "test@example.com"));
        table.insert((10, "test10", "test@example.com"));
        table.insert((11, "test11", "test@example.com"));
        table.insert((12, "test12", "test@example.com"));
        table.insert((13, "test13", "test@example.com"));
        table.insert((14, "test14", "test@example.com"));
        table.insert((15, "test15", "test@example.com"));
        table.insert((16, "test16", "test@example.com"));
        table.insert((17, "test17", "test@example.com"));

        assert_eq!(table.num_rows, 17);
        assert!(table.pages[0].is_some());
        assert!(table.pages[1].is_some());
        for i in 2..TABLE_MAX_PAGES {
            assert!(table.pages[i].is_none());
        }

        for i in 0..14u8 {
            match table.page(0).and_then(|page| page.row(i as usize)) {
                Some(Row {
                    ref id,
                    ref username,
                    ref email,
                }) => {
                    assert_eq!(*id, i as u32 + 1);
                    assert_eq!(&username[0..4], b"test");
                    assert_eq!(&username[4..6], format!("{:02}", i + 1).as_bytes());
                    assert_eq!(&username[6..32], &[0; 26]);
                    assert_eq!(&email[0..16], b"test@example.com");
                    assert_eq!(&email[16..255], &[0; 239]);
                }
                None => panic!("no table.page(0).and_then(page.row(i))"),
            }
        }

        for j in 14..17u8 {
            let i = j - 14;
            match table.page(1).and_then(|page| page.row(i as usize)) {
                Some(Row {
                    ref id,
                    ref username,
                    ref email,
                }) => {
                    assert_eq!(*id, j as u32 + 1);
                    assert_eq!(&username[0..4], b"test");
                    assert_eq!(&username[4..6], format!("{:02}", j + 1).as_bytes());
                    assert_eq!(&username[6..32], &[0; 26]);
                    assert_eq!(&email[0..16], b"test@example.com");
                    assert_eq!(&email[16..255], &[0; 239]);
                }
                None => panic!("no table.page(1).and_then(page.row(i))"),
            }
        }

        match table.pages[0] {
            Some(ref page) => {
                for i in 0..14u8 {
                    let bytes = &page.bytes[292 * i as usize..292 * i as usize + 292];

                    assert_eq!(&bytes[0..4], &[i + 1, 0, 0, 0]);
                    assert_eq!(&bytes[4..8], b"test");
                    assert_eq!(&bytes[8..10], format!("{:02}", i + 1).as_bytes());
                    assert_eq!(&bytes[10..36], &[0; 26]);
                    assert_eq!(&bytes[36..52], b"test@example.com");
                    assert_eq!(&bytes[52..291], &[0; 239]);
                }
            }
            None => panic!("no pages[0]"),
        }

        match table.pages[1] {
            Some(ref page) => {
                for j in 14..17u8 {
                    let i = j - 14;
                    let bytes = &page.bytes[292 * i as usize..292 * i as usize + 292];

                    assert_eq!(&bytes[0..4], &[j + 1, 0, 0, 0]);
                    assert_eq!(&bytes[4..8], b"test");
                    assert_eq!(&bytes[8..10], format!("{:02}", j + 1).as_bytes());
                    assert_eq!(&bytes[10..36], &[0; 26]);
                    assert_eq!(&bytes[36..52], b"test@example.com");
                    assert_eq!(&bytes[52..291], &[0; 239]);
                }
            }
            None => panic!("no pages[1]"),
        }
    }
}
