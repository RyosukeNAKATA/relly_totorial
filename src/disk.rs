pub struct DiskManager {
    // ヒープファイルディスクリプタ
    heap_file: File,
    // 採番するページのIDを決めるカウンタ
    next_page_id: u64,
}

pub struct PageId(pub u64);

impl DiskManager {
    // コンストラクタ
    pub fn new(data_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {heap_file, next_page_id,})
    }
    // ファイルパスを指定して書く
    pub fn open(data_file_path: impl Asref<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new().read(true).write(true).create(true).open(heap_file_path)?;
        Self::new(heap_file)
    }
    // 新しいページIDを採番する
    pub fn allocate_page(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }
    // ページのデータを読み出す
    pub fn read_page_data(&mut self, page_id: PageId, date: &[u8]) -> io::Result<()> {
        // オフセットを計算
        let offset = PAGE_SiZE as u64 * page_id.to_u64();
        // ページ先頭へシーク - ファイル先頭からoffsetバイト目をシークする
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // データを読み出す
        self.heap_file.read_exact(data)
    }
    // データをページに書き出す
    pub fn write_page_data(&mut self, page_id: PageId, date: &[u8]) -> io::Result<()> {
        // オフセットを計算
        let offset = PAGE_SiZE as u64 * page_id.to_u64();
        // ページ先頭へシーク - ファイル先頭からoffsetバイト目をシークする
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // データを書き込む
        self.heap_file.write_all(data)
    }
}