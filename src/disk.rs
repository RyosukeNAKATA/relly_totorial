pub struct DiskManager {
    // ヒープファイルディスクリプタ
    heap_file: File,
    // 採番するページのIDを決めるカウンタ
    next_page_id: u64,
}
