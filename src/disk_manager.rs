use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io;
use std::io::{Seek, SeekFrom, Write, Read};

const PAGE_SIZE: usize = 4096;

pub struct PageId (pub u64);

impl PageId {
    pub fn to_64(&self) -> u64 {
        self.0
    }
}

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }
    pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new().read(true).write(true).create(true).open(heap_file_path)?;
        Self::new(heap_file)
    }
    pub fn allocate_page(&mut self) -> PageId {
        self.next_page_id += 1;
        PageId (self.next_page_id - 1)
    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.to_64();
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.write_all(data)
    }
    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.to_64();
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.read_exact(data)
    }
}